//! tg-ws-proxy-rs — Telegram MTProto WebSocket Bridge Proxy
//!
//! Listens for Telegram Desktop MTProto connections and forwards them through
//! WebSocket tunnels to Telegram's DC servers, bypassing networks that block
//! direct Telegram TCP traffic.
//!
//! # Architecture
//!
//! ```
//! Telegram Desktop → MTProto (TCP 1443) → tg-ws-proxy-rs → WS (TLS 443) → Telegram DC
//! ```
//!
//! See [`proxy`] for the connection handling logic and [`crypto`] for the
//! MTProto obfuscation details.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use tracing::{error, info, warn};

#[cfg(feature = "gui")]
use parking_lot::RwLock;

// ── File-descriptor budget helpers ───────────────────────────────────────────

/// Read the soft per-process open-file limit from `/proc/self/limits` (Linux).
/// Falls back to 1 024 on other platforms or when the file cannot be parsed.
fn soft_nofile_limit() -> usize {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/self/limits") {
            for line in content.lines() {
                // Example line:
                //   Max open files            1024                 4096                 files
                if line.starts_with("Max open files") {
                    if let Some(soft_str) = line.split_whitespace().nth(3) {
                        if soft_str == "unlimited" {
                            return usize::MAX;
                        }
                        if let Ok(n) = soft_str.parse::<usize>() {
                            return n;
                        }
                    }
                }
            }
        }
    }

    1024 // conservative fallback for non-Linux or parse failures
}

/// Compute a safe default for the maximum number of concurrent connections
/// given the system FD limit and pool configuration.
///
/// FD budget:
///   1 (listener) + pool_size × dc_buckets × 2 (idle + one refill per bucket)
///   + 32 (Tokio runtime, stdio, safety margin)
///   + max_connections × 2 (one client socket + one outbound socket per conn)
///
/// Rearranging for max_connections:
///   max_connections = (fd_limit − reserved) / 2
fn auto_max_connections(fd_limit: usize, pool_size: usize, dc_buckets: usize) -> usize {
    if fd_limit == usize::MAX {
        // Unlimited FDs: cap at a large but sane value.
        return 512;
    }

    let reserved = 1 + pool_size * dc_buckets * 2 + 32;

    (fd_limit.saturating_sub(reserved) / 2).max(4)
}

mod config;
mod crypto;
mod pool;
mod proxy;
mod splitter;
mod ws_client;
mod logger;

#[cfg(feature = "gui")]
mod gui;

use config::Config;
use pool::WsPool;

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls ring CryptoProvider");

    let config = Config::from_args();

    // ── Logging ──────────────────────────────────────────────────────────
    let log_level = if config.quiet {
        "off"
    } else if config.verbose {
        "debug"
    } else {
        "info"
    };

    #[cfg(feature = "gui")]
    let log_messages = Arc::new(RwLock::new(Vec::new()));

    #[cfg(feature = "gui")]
    {
        logger::init_gui_logging(log_level, log_messages.clone())
            .expect("failed to initialize logging");
    }

    #[cfg(not(feature = "gui"))]
    {
        logger::init_file_logging(log_level);
    }

    // ── Bind the server socket ────────────────────────────────────────────
    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("invalid listen address");

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("cannot bind {}: {}", addr, e));

    // ── FD budget & effective max_connections ────────────────────────────
    // Each active connection uses 2 FDs: the accepted client socket and the
    // outbound connection to Telegram (WS or TCP fallback).  The pool adds
    // pool_size × dc_buckets × 2 FDs (idle + one in-flight refill per bucket).
    // Auto-compute a safe default when the user has not set --max-connections,
    // so the proxy stays within the process's soft file-descriptor limit.
    let fd_limit = soft_nofile_limit();
    let dc_redirects = config.dc_redirects();
    let dc_buckets = dc_redirects.len() * 2; // non-media + media per DC
    let max_connections = match config.max_connections {
        Some(n) => {
            let safe = auto_max_connections(fd_limit, config.pool_size, dc_buckets);
            if n > safe {
                warn!(
                    "max-connections={} may exceed the safe limit for this system's \
                     FD budget (fd-limit={}, recommended ≤{}). \
                     Consider raising `ulimit -n` or reducing --max-connections.",
                    n, fd_limit, safe
                );
            }
            n
        }
        None => auto_max_connections(fd_limit, config.pool_size, dc_buckets),
    };

    // ── Print startup banner ──────────────────────────────────────────────
    let secret = config.secret.as_deref().unwrap_or("");

    let link_host = config.link_host();
    let tg_link = format!(
        "tg://proxy?server={}&port={}&secret=dd{}",
        link_host, config.port, secret
    );

    info!("{}", "=".repeat(60));
    info!("  Telegram MTProto WS Bridge Proxy  (tg-ws-proxy-rs)");
    info!("  Listening on   {}:{}", config.host, config.port);
    info!("  Secret:        {}", secret);
    info!("  Target DC IPs:");
    let mut dcs: Vec<_> = dc_redirects.iter().collect();
    dcs.sort_by_key(|(k, _)| *k);
    for (dc, ip) in &dcs {
        info!("    DC{}: {}", dc, ip);
    }

    if config.skip_tls_verify {
        info!("  ⚠  TLS certificate verification DISABLED");
    }

    if !config.mtproto_proxies.is_empty() {
        info!("  Upstream MTProto proxies (WS fallback):");
        for p in &config.mtproto_proxies {
            info!("    {}:{}", p.host, p.port);
        }
    }

    info!(
        "  Max connections: {} (fd-limit: {})",
        max_connections, fd_limit
    );
    info!("{}", "=".repeat(60));
    info!("  Telegram proxy link (use this on all devices):");
    info!("    {}", tg_link);

    if link_host != config.host {
        info!(
            "  ℹ  Link uses auto-detected IP {}. \
             Use --link-ip <IP> to override.",
            link_host
        );
    } else if matches!(config.host.as_str(), "127.0.0.1" | "::1") {
        warn!(
            "  ⚠  Link shows {} — only the local machine can use this link. \
             Run with --host 0.0.0.0 (or --link-ip <router-LAN-IP>) \
             so other devices on the network can connect.",
            config.host
        );
    }
    info!("{}", "=".repeat(60));

    // ── Connection pool warm-up ───────────────────────────────────────────
    let pool = Arc::new(WsPool::new(config.pool_size));
    let semaphore = Arc::new(Semaphore::new(max_connections));
    
    {
        let pool_clone = pool.clone();
        let config_clone = config.clone();
        tokio::spawn(async move {
            pool_clone.warmup(&config_clone).await;
        });
    }

    #[cfg(feature = "gui")]
    {
        let stats = Arc::new(RwLock::new(gui::ProxyStats::default()));
        
        // Prepare GUI display strings
        let mut config_lines = vec![
            format!("Listening: {}:{}", config.host, config.port),
            format!("Secret: {}", secret),
        ];
        
        if config.skip_tls_verify {
            config_lines.push("⚠ TLS verification: DISABLED".to_string());
        }
        
        config_lines.push(format!("Max connections: {}", max_connections));
        
        let config_display = config_lines.join("\n");
        
        // Clone for async task
        let listener_clone = listener;
        let config_clone = config.clone();
        let pool_clone = pool.clone();
        let stats_clone = stats.clone();
        let semaphore_clone = semaphore.clone();
        
        // Spawn proxy server in background
        tokio::spawn(async move {
            run_proxy_server(
                listener_clone,
                config_clone,
                pool_clone,
                stats_clone,
                semaphore_clone,
                max_connections,
            )
            .await;
        });
        
        // Run GUI on main thread
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([600.0, 700.0])
                .with_min_inner_size([500.0, 600.0])
                .with_icon(eframe::icon_data::from_png_bytes(&[]).unwrap_or_default()),
            ..Default::default()
        };
        
        let app = gui::ProxyApp::new(config_display, tg_link, stats, log_messages);
        
        let _ = eframe::run_native(
            "Telegram WS Proxy",
            native_options,
            Box::new(|_cc| Ok(Box::new(app))),
        );
    }

    #[cfg(not(feature = "gui"))]
    {
        run_proxy_server(listener, config, pool, semaphore, max_connections).await;
    }
}

#[cfg(feature = "gui")]
async fn run_proxy_server(
    listener: TcpListener,
    config: Config,
    pool: Arc<WsPool>,
    stats: Arc<RwLock<gui::ProxyStats>>,
    semaphore: Arc<Semaphore>,
    _max_connections: usize,
) {
    const EMFILE: i32 = 24;
    const ENFILE: i32 = 23;
    
    loop {
        let permit = Arc::clone(&semaphore)
            .acquire_owned()
            .await
            .expect("semaphore closed unexpectedly");

        match listener.accept().await {
            Ok((stream, peer_addr)) => {
                {
                    let mut s = stats.write();
                    s.active_connections += 1;
                    s.total_connections += 1;
                }
                
                let cfg = config.clone();
                let pool = pool.clone();
                let stats_clone = stats.clone();
                
                tokio::spawn(async move {
                    let _permit = permit;
                    proxy::handle_client(stream, peer_addr, cfg, pool).await;
                    
                    let mut s = stats_clone.write();
                    s.active_connections = s.active_connections.saturating_sub(1);
                });
            }
            Err(e) => {
                if matches!(e.raw_os_error(), Some(EMFILE) | Some(ENFILE)) {
                    warn!("accept error: {} — backing off to allow FDs to free", e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                } else {
                    error!("accept error: {}", e);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }
        }
    }
}

#[cfg(not(feature = "gui"))]
async fn run_proxy_server(
    listener: TcpListener,
    config: Config,
    pool: Arc<WsPool>,
    semaphore: Arc<Semaphore>,
    _max_connections: usize,
) {
    const EMFILE: i32 = 24;
    const ENFILE: i32 = 23;
    
    loop {
        let permit = Arc::clone(&semaphore)
            .acquire_owned()
            .await
            .expect("semaphore closed unexpectedly");

        match listener.accept().await {
            Ok((stream, peer_addr)) => {
                let cfg = config.clone();
                let pool = pool.clone();
                tokio::spawn(async move {
                    let _permit = permit;
                    proxy::handle_client(stream, peer_addr, cfg, pool).await;
                });
            }
            Err(e) => {
                if matches!(e.raw_os_error(), Some(EMFILE) | Some(ENFILE)) {
                    warn!("accept error: {} — backing off to allow FDs to free", e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                } else {
                    error!("accept error: {}", e);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }
        }
    }
}
