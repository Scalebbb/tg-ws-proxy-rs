//! Automatic DC IP updater with fallback to hardcoded addresses

use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn, error, debug};

/// Fallback DC IPs (hardcoded, always available)
pub fn fallback_dc_ips() -> HashMap<u32, String> {
    [
        (1, "149.154.175.53"),
        (2, "149.154.167.51"),
        (3, "149.154.175.100"),
        (4, "149.154.167.91"),
        (5, "91.108.56.130"),
        (203, "149.154.167.40"),
    ]
    .iter()
    .map(|(k, v)| (*k, v.to_string()))
    .collect()
}

/// Sources for fetching DC IPs
const DC_IP_SOURCES: &[&str] = &[
    "https://core.telegram.org/getProxyConfig",
    "https://telegram.org/getProxyConfig",
];

/// Telegram DC configuration from API
#[derive(Debug, Clone)]
pub struct DcConfig {
    pub ips: HashMap<u32, String>,
    pub timestamp: std::time::SystemTime,
}

impl DcConfig {
    pub fn fallback() -> Self {
        Self {
            ips: fallback_dc_ips(),
            timestamp: std::time::SystemTime::now(),
        }
    }
}

/// Fetch DC IPs from Telegram API
pub async fn fetch_dc_ips() -> Option<HashMap<u32, String>> {
    info!("Fetching latest DC IPs from Telegram...");
    
    // Try each source
    for source in DC_IP_SOURCES {
        debug!("Trying source: {}", source);
        
        match fetch_from_source(source).await {
            Some(ips) if !ips.is_empty() => {
                info!("Successfully fetched {} DC IPs from {}", ips.len(), source);
                return Some(ips);
            }
            Some(_) => {
                warn!("Source {} returned empty DC list", source);
            }
            None => {
                warn!("Failed to fetch from {}", source);
            }
        }
    }
    
    // Try DNS resolution as fallback
    if let Some(ips) = fetch_via_dns().await {
        info!("Successfully resolved {} DC IPs via DNS", ips.len());
        return Some(ips);
    }
    
    error!("All DC IP sources failed, using fallback IPs");
    None
}

/// Fetch DC config from a specific source
async fn fetch_from_source(url: &str) -> Option<HashMap<u32, String>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .ok()?;
    
    let response = client.get(url).send().await.ok()?;
    
    if !response.status().is_success() {
        return None;
    }
    
    let text = response.text().await.ok()?;
    parse_proxy_config(&text)
}

/// Parse Telegram proxy config format
fn parse_proxy_config(text: &str) -> Option<HashMap<u32, String>> {
    let mut ips = HashMap::new();
    
    // Try JSON format first
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
        if let Some(dcs) = json.get("dc_options").and_then(|v| v.as_array()) {
            for dc in dcs {
                if let (Some(id), Some(ip)) = (
                    dc.get("id").and_then(|v| v.as_u64()),
                    dc.get("ip_address").and_then(|v| v.as_str()),
                ) {
                    ips.insert(id as u32, ip.to_string());
                }
            }
        }
    }
    
    // Try plain text format: "DC_ID:IP_ADDRESS"
    if ips.is_empty() {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((dc_str, ip_str)) = line.split_once(':') {
                if let Ok(dc) = dc_str.trim().parse::<u32>() {
                    let ip = ip_str.trim().to_string();
                    if ip.parse::<std::net::IpAddr>().is_ok() {
                        ips.insert(dc, ip);
                    }
                }
            }
        }
    }
    
    if ips.is_empty() {
        None
    } else {
        Some(ips)
    }
}

/// Fetch DC IPs via DNS resolution
async fn fetch_via_dns() -> Option<HashMap<u32, String>> {
    let mut ips = HashMap::new();
    
    // Known Telegram domains for each DC
    let dc_domains = [
        (1, "pluto.web.telegram.org"),
        (2, "venus.web.telegram.org"),
        (3, "aurora.web.telegram.org"),
        (4, "vesta.web.telegram.org"),
        (5, "flora.web.telegram.org"),
    ];
    
    for (dc, domain) in dc_domains {
        if let Ok(addrs) = tokio::net::lookup_host(format!("{}:443", domain)).await {
            if let Some(addr) = addrs.into_iter().next() {
                ips.insert(dc, addr.ip().to_string());
                debug!("Resolved DC{}: {} -> {}", dc, domain, addr.ip());
            }
        }
    }
    
    if ips.len() >= 3 {
        Some(ips)
    } else {
        None
    }
}

/// Update DC IPs with automatic fallback
pub async fn get_dc_ips_with_fallback() -> HashMap<u32, String> {
    match fetch_dc_ips().await {
        Some(ips) => {
            info!("Using fresh DC IPs");
            ips
        }
        None => {
            warn!("Using fallback DC IPs");
            fallback_dc_ips()
        }
    }
}

/// Background task to periodically update DC IPs
pub async fn dc_updater_task(
    dc_config: std::sync::Arc<tokio::sync::RwLock<DcConfig>>,
    update_interval: Duration,
) {
    loop {
        tokio::time::sleep(update_interval).await;
        
        info!("Checking for DC IP updates...");
        
        match fetch_dc_ips().await {
            Some(new_ips) => {
                let mut config = dc_config.write().await;
                
                // Check if IPs changed
                let changed = new_ips != config.ips;
                
                if changed {
                    info!("DC IPs updated:");
                    for (dc, ip) in &new_ips {
                        if let Some(old_ip) = config.ips.get(dc) {
                            if old_ip != ip {
                                info!("  DC{}: {} -> {}", dc, old_ip, ip);
                            }
                        } else {
                            info!("  DC{}: {} (new)", dc, ip);
                        }
                    }
                    
                    config.ips = new_ips;
                    config.timestamp = std::time::SystemTime::now();
                } else {
                    debug!("DC IPs unchanged");
                }
            }
            None => {
                warn!("Failed to update DC IPs, keeping current configuration");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_ips() {
        let ips = fallback_dc_ips();
        assert!(ips.len() >= 5);
        assert!(ips.contains_key(&1));
        assert!(ips.contains_key(&2));
    }

    #[test]
    fn test_parse_plain_text() {
        let text = "1:149.154.175.53\n2:149.154.167.51\n# comment\n3:149.154.175.100";
        let ips = parse_proxy_config(text).unwrap();
        assert_eq!(ips.len(), 3);
        assert_eq!(ips.get(&1).unwrap(), "149.154.175.53");
    }

    #[test]
    fn test_parse_json() {
        let text = r#"{"dc_options":[{"id":1,"ip_address":"149.154.175.53"},{"id":2,"ip_address":"149.154.167.51"}]}"#;
        let ips = parse_proxy_config(text).unwrap();
        assert_eq!(ips.len(), 2);
        assert_eq!(ips.get(&1).unwrap(), "149.154.175.53");
    }
}
