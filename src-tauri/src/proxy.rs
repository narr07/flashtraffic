use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProxyItem {
    pub id: String,
    pub address: String, // e.g. 192.168.1.1:8080 or host:port:user:pass
    pub protocol: String, // http, https, socks4, socks5
    pub speed_ms: Option<u64>,
    pub is_alive: bool,
    pub anonymity: String, // Transparent, Anonymous, Elite
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyTestResult {
    pub address: String,
    pub is_alive: bool,
    pub latency_ms: u64,
    pub error_msg: Option<String>,
}

/// Tests a single proxy asynchronously with a strict timeout
pub async fn test_single_proxy(proxy_str: &str, target_url: &str) -> ProxyTestResult {
    let start = Instant::now();
    let parts: Vec<&str> = proxy_str.trim().split(':').collect();
    
    let proxy_url = if parts.len() >= 4 {
        format!("http://{}:{}@{}:{}", parts[2], parts[3], parts[0], parts[1])
    } else if proxy_str.starts_with("http://") || proxy_str.starts_with("https://") || proxy_str.starts_with("socks5://") {
        proxy_str.to_string()
    } else {
        format!("http://{}", proxy_str)
    };

    let client_builder = match reqwest::Proxy::all(&proxy_url) {
        Ok(p) => reqwest::Client::builder()
            .proxy(p)
            .timeout(std::time::Duration::from_secs(5)),
        Err(e) => {
            return ProxyTestResult {
                address: proxy_str.to_string(),
                is_alive: false,
                latency_ms: 0,
                error_msg: Some(format!("Invalid proxy format: {}", e)),
            };
        }
    };

    match client_builder.build() {
        Ok(client) => {
            match client.get(target_url).send().await {
                Ok(resp) => {
                    let elapsed = start.elapsed().as_millis() as u64;
                    ProxyTestResult {
                        address: proxy_str.to_string(),
                        is_alive: resp.status().is_success() || resp.status().is_redirection(),
                        latency_ms: elapsed,
                        error_msg: None,
                    }
                }
                Err(err) => ProxyTestResult {
                    address: proxy_str.to_string(),
                    is_alive: false,
                    latency_ms: start.elapsed().as_millis() as u64,
                    error_msg: Some(err.to_string()),
                },
            }
        }
        Err(err) => ProxyTestResult {
            address: proxy_str.to_string(),
            is_alive: false,
            latency_ms: 0,
            error_msg: Some(err.to_string()),
        },
    }
}

/// Batch tests up to 50 proxies concurrently in Tokio thread pool
pub async fn test_proxy_batch(proxies: Vec<String>, test_url: String) -> Vec<ProxyTestResult> {
    let mut handles = Vec::new();
    let url = if test_url.is_empty() {
        "https://httpbin.org/ip".to_string()
    } else {
        test_url
    };

    for p in proxies {
        let u = url.clone();
        handles.push(tokio::spawn(async move {
            test_single_proxy(&p, &u).await
        }));
    }

    let mut results = Vec::new();
    for h in handles {
        if let Ok(res) = h.await {
            results.push(res);
        }
    }
    results
}

/// Fetches any web page or proxy list URL and automatically extracts valid IP:Port pairs
pub async fn fetch_proxies_from_url(url: &str) -> Result<Vec<String>, String> {
    let target_url = if url.contains("proxyscrape.com") && !url.contains("api.proxyscrape.com") {
        "https://api.proxyscrape.com/v2/?request=displayproxies&protocol=http&timeout=10000&country=all&ssl=all&anonymity=all"
    } else {
        url
    };

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(12))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let resp = client.get(target_url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch URL '{}': {}", url, e))?;

    if !resp.status().is_success() {
        return Err(format!("Server returned HTTP status: {}", resp.status()));
    }

    let body = resp.text().await.map_err(|e| format!("Failed to read response body: {}", e))?;
    Ok(extract_proxies_from_text(&body))
}

pub fn extract_proxies_from_text(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Regex 1: Exact IP:Port pattern (e.g. raw text lists, FineProxy 130.110.103.245:3128)
    if let Ok(re_direct) = regex::Regex::new(r"\b((?:[0-9]{1,3}\.){3}[0-9]{1,3}):([0-9]{2,5})\b") {
        for cap in re_direct.captures_iter(text) {
            let ip = &cap[1];
            let port_str = &cap[2];
            if is_valid_ip(ip) && is_valid_port(port_str) {
                let entry = format!("{}:{}", ip, port_str);
                if seen.insert(entry.clone()) {
                    found.push(entry);
                }
            }
        }
    }

    // Regex 2: Table structure or separated IP and port (e.g. Proxora, FreeProxyLists, RoundProxies)
    if let Ok(re_sep) = regex::Regex::new(r#"((?:[0-9]{1,3}\.){3}[0-9]{1,3})[^\d]{1,60}?([0-9]{2,5})"#) {
        for cap in re_sep.captures_iter(text) {
            let ip = &cap[1];
            let port_str = &cap[2];
            if is_valid_ip(ip) && is_valid_port(port_str) {
                let entry = format!("{}:{}", ip, port_str);
                if seen.insert(entry.clone()) {
                    found.push(entry);
                }
            }
        }
    }

    found
}

fn is_valid_ip(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    for p in parts {
        match p.parse::<u8>() {
            Ok(_) => {}
            Err(_) => return false,
        }
    }
    if ip == "0.0.0.0" || ip == "127.0.0.1" || ip == "255.255.255.255" {
        return false;
    }
    true
}

fn is_valid_port(port_str: &str) -> bool {
    if let Ok(p) = port_str.parse::<u16>() {
        p > 0 && p != 2026 // Avoid year artifact
    } else {
        false
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_extract_proxies() {
        let sample = "Here are some proxies: 130.110.103.245:3128 and 3.211.120.181:443 and invalid 999.999.999.999:80";
        let proxies = extract_proxies_from_text(sample);
        assert_eq!(proxies.len(), 2);
        assert_eq!(proxies[0], "130.110.103.245:3128");
        assert_eq!(proxies[1], "3.211.120.181:443");
    }
}
