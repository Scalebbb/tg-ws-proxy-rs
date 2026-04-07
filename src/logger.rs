//! File-based logging module

use std::io::Write;
use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[cfg(feature = "gui")]
use parking_lot::RwLock;

#[cfg(feature = "gui")]
pub struct GuiLogWriter {
    messages: Arc<RwLock<Vec<String>>>,
}

#[cfg(feature = "gui")]
impl GuiLogWriter {
    pub fn new(messages: Arc<RwLock<Vec<String>>>) -> Self {
        Self { messages }
    }
}

#[cfg(feature = "gui")]
impl Write for GuiLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(s) = std::str::from_utf8(buf) {
            let mut msgs = self.messages.write();
            msgs.push(s.trim_end().to_string());
            if msgs.len() > 1000 {
                msgs.drain(0..500);
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn init_file_logging(log_level: &str) {
    let log_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("logs");

    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::daily(log_dir, "tg-ws-proxy.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| log_level.into()),
                ),
        )
        .init();

    // Leak the guard to keep logging active for the entire program lifetime
    std::mem::forget(_guard);
}

#[cfg(feature = "gui")]
pub fn init_gui_logging(
    log_level: &str,
    log_messages: Arc<RwLock<Vec<String>>>,
) -> std::io::Result<()> {
    let log_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("logs");

    std::fs::create_dir_all(&log_dir)?;

    let file_appender = tracing_appender::rolling::daily(log_dir, "tg-ws-proxy.log");
    let (non_blocking_file, _file_guard) = tracing_appender::non_blocking(file_appender);

    let gui_writer = GuiLogWriter::new(log_messages);
    let (non_blocking_gui, _gui_guard) = tracing_appender::non_blocking(gui_writer);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_file)
                .with_ansi(false)
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| log_level.into()),
                ),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_gui)
                .with_ansi(false)
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| log_level.into()),
                ),
        )
        .init();

    // Leak the guards to keep logging active
    std::mem::forget(_file_guard);
    std::mem::forget(_gui_guard);

    Ok(())
}
