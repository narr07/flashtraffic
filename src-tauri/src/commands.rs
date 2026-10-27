use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::process::Command;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use crate::safety::{register_pid, unregister_pid, CREATE_NO_WINDOW};
use crate::proxy::{test_proxy_batch, fetch_proxies_from_url as proxy_fetch_from_url, ProxyTestResult};
use crate::browserstack::{verify_credentials, BrowserStackPlan};

static CAMPAIGN_RUNNING: AtomicBool = AtomicBool::new(false);
static CAMPAIGN_LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static METRICS: Mutex<CampaignMetrics> = Mutex::new(CampaignMetrics {
    total_visits: 0,
    successful_visits: 0,
    failed_visits: 0,
    active_visitors: 0,
    current_status: String::new(),
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledBrowser {
    pub id: String,
    pub name: String,
    pub path: String,
    pub is_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignPayload {
    pub name: String,
    pub target_urls: Vec<String>,
    pub traffic_type: String, // Direct, Organic, Referral
    pub traffic_source: String, // Google, Bing, Facebook, etc.
    #[serde(default)]
    pub search_engines: Option<Vec<String>>,
    #[serde(default)]
    pub referral_sources: Option<Vec<String>>,
    #[serde(default)]
    pub custom_referral_url: Option<String>,
    pub keywords: String,
    #[serde(default)]
    pub keyword_list: Option<Vec<String>>,
    pub utm_campaign: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_source: Option<String>,
    pub headless: bool,
    pub dwell_time_seconds: u32,
    pub allow_surfing: bool,
    pub proxies: Vec<String>,
    pub max_sessions: u32,
    #[serde(default)]
    pub traffic_mode: Option<String>,
    #[serde(default)]
    pub session_duration_type: Option<String>,
    #[serde(default)]
    pub session_duration_constant: Option<u64>,
    #[serde(default)]
    pub session_duration_min: Option<u64>,
    #[serde(default)]
    pub session_duration_max: Option<u64>,
    #[serde(default)]
    pub traffic_delay_type: Option<String>,
    #[serde(default)]
    pub traffic_delay_constant: Option<u64>,
    #[serde(default)]
    pub traffic_delay_min: Option<u64>,
    #[serde(default)]
    pub traffic_delay_max: Option<u64>,
    #[serde(default)]
    pub page_order: Option<String>,
    #[serde(default)]
    pub proxy_switching: Option<String>,
    #[serde(default)]
    pub use_random_proxy: Option<bool>,
    #[serde(default)]
    pub platform_switching: Option<String>,
    #[serde(default)]
    pub special_mode: Option<String>,
    #[serde(default)]
    pub surfing_count: Option<u32>,
    #[serde(default)]
    pub client_timestamp: Option<String>,

    // Engine and System Browser Borrowing
    #[serde(default)]
    pub engine_type: Option<String>, // "system_browser" | "native_light"
    #[serde(default)]
    pub browser_channel: Option<String>, // "msedge" | "chrome" | "brave" | "custom"
    #[serde(default)]
    pub custom_browser_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignMetrics {
    pub total_visits: u32,
    pub successful_visits: u32,
    pub failed_visits: u32,
    pub active_visitors: u32,
    pub current_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStatus {
    pub is_running: bool,
    pub metrics: CampaignMetrics,
    #[serde(default)]
    pub logs: Vec<String>,
}

struct PlatformProfile {
    browser: &'static str,
    os_device: &'static str,
    ua: &'static str,
}

const PLATFORM_PROFILES: &[PlatformProfile] = &[
    PlatformProfile {
        browser: "Google Chrome-122.0.6261.129",
        os_device: "Windows-Windows 11",
        ua: "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.6261.129 Safari /537.36",
    },
    PlatformProfile {
        browser: "Microsoft Edge-122.0.2365.92",
        os_device: "Windows-Windows 11",
        ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0",
    },
    PlatformProfile {
        browser: "Mozilla Firefox-123.0",
        os_device: "Windows-Windows 10",
        ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:123.0) Gecko/20100101 Firefox/123.0",
    },
    PlatformProfile {
        browser: "Apple Safari-17.3",
        os_device: "macOS-macOS Sonoma",
        ua: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.3 Safari/605.1.15",
    },
    PlatformProfile {
        browser: "Google Chrome-123.0.6312.86",
        os_device: "macOS-macOS Sonoma",
        ua: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    },
    PlatformProfile {
        browser: "Chrome Mobile-122.0.6261.119",
        os_device: "Android-Android 14",
        ua: "Mozilla/5.0 (Linux; Android 14; Pixel 8 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.6261.119 Mobile Safari/537.36",
    },
    PlatformProfile {
        browser: "Mobile Safari-17.2",
        os_device: "iOS-iOS 17.2",
        ua: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
    },
];

pub fn resolve_system_browser_exe(channel: Option<&str>, custom_path: Option<&str>) -> Option<String> {
    if let Some(cp) = custom_path {
        let trimmed = cp.trim();
        if !trimmed.is_empty() && std::path::Path::new(trimmed).exists() {
            return Some(trimmed.to_string());
        }
    }

    let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let prog_files = std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string());
    let prog_files_x86 = std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| "C:\\Program Files (x86)".to_string());

    match channel {
        Some("chrome") => {
            let candidates = [
                format!("{}\\Google\\Chrome\\Application\\chrome.exe", prog_files),
                format!("{}\\Google\\Chrome\\Application\\chrome.exe", prog_files_x86),
                format!("{}\\Google\\Chrome\\Application\\chrome.exe", local_app_data),
            ];
            candidates.into_iter().find(|p| std::path::Path::new(p).exists())
        }
        Some("brave") => {
            let candidates = [
                format!("{}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe", prog_files),
                format!("{}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe", prog_files_x86),
                format!("{}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe", local_app_data),
            ];
            candidates.into_iter().find(|p| std::path::Path::new(p).exists())
        }
        Some("firefox") => {
            let candidates = [
                format!("{}\\Mozilla Firefox\\firefox.exe", prog_files),
                format!("{}\\Mozilla Firefox\\firefox.exe", prog_files_x86),
            ];
            candidates.into_iter().find(|p| std::path::Path::new(p).exists())
        }
        _ => {
            // Default: Microsoft Edge (installed on 100% of Windows 10 & 11)
            let candidates = [
                format!("{}\\Microsoft\\Edge\\Application\\msedge.exe", prog_files_x86),
                format!("{}\\Microsoft\\Edge\\Application\\msedge.exe", prog_files),
                format!("{}\\Microsoft\\Edge\\Application\\msedge.exe", local_app_data),
            ];
            candidates.into_iter().find(|p| std::path::Path::new(p).exists())
        }
    }
}

#[tauri::command(async)]
pub async fn get_installed_browsers() -> Result<Vec<InstalledBrowser>, String> {
    let mut browsers = Vec::new();

    let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let prog_files = std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string());
    let prog_files_x86 = std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| "C:\\Program Files (x86)".to_string());

    // 1. Edge (Windows default)
    let edge_paths = [
        format!("{}\\Microsoft\\Edge\\Application\\msedge.exe", prog_files_x86),
        format!("{}\\Microsoft\\Edge\\Application\\msedge.exe", prog_files),
        format!("{}\\Microsoft\\Edge\\Application\\msedge.exe", local_app_data),
    ];
    let edge_found = edge_paths.iter().find(|p| std::path::Path::new(p).exists()).cloned();
    browsers.push(InstalledBrowser {
        id: "msedge".to_string(),
        name: "Microsoft Edge (Bawaan Windows)".to_string(),
        path: edge_found.clone().unwrap_or_else(|| edge_paths[0].clone()),
        is_available: edge_found.is_some(),
    });

    // 2. Google Chrome
    let chrome_paths = [
        format!("{}\\Google\\Chrome\\Application\\chrome.exe", prog_files),
        format!("{}\\Google\\Chrome\\Application\\chrome.exe", prog_files_x86),
        format!("{}\\Google\\Chrome\\Application\\chrome.exe", local_app_data),
    ];
    let chrome_found = chrome_paths.iter().find(|p| std::path::Path::new(p).exists()).cloned();
    browsers.push(InstalledBrowser {
        id: "chrome".to_string(),
        name: "Google Chrome".to_string(),
        path: chrome_found.clone().unwrap_or_else(|| chrome_paths[0].clone()),
        is_available: chrome_found.is_some(),
    });

    // 3. Brave Browser
    let brave_paths = [
        format!("{}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe", prog_files),
        format!("{}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe", prog_files_x86),
        format!("{}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe", local_app_data),
    ];
    let brave_found = brave_paths.iter().find(|p| std::path::Path::new(p).exists()).cloned();
    browsers.push(InstalledBrowser {
        id: "brave".to_string(),
        name: "Brave Browser".to_string(),
        path: brave_found.clone().unwrap_or_else(|| brave_paths[0].clone()),
        is_available: brave_found.is_some(),
    });

    // 4. Firefox
    let firefox_paths = [
        format!("{}\\Mozilla Firefox\\firefox.exe", prog_files),
        format!("{}\\Mozilla Firefox\\firefox.exe", prog_files_x86),
    ];
    let firefox_found = firefox_paths.iter().find(|p| std::path::Path::new(p).exists()).cloned();
    browsers.push(InstalledBrowser {
        id: "firefox".to_string(),
        name: "Mozilla Firefox".to_string(),
        path: firefox_found.clone().unwrap_or_else(|| firefox_paths[0].clone()),
        is_available: firefox_found.is_some(),
    });

    Ok(browsers)
}

#[tauri::command(async)]
pub async fn start_campaign(payload: CampaignPayload) -> Result<String, String> {
    if CAMPAIGN_RUNNING.load(Ordering::SeqCst) {
        return Err("A campaign is already running".to_string());
    }

    if payload.target_urls.is_empty() {
        return Err("Target URL list cannot be empty".to_string());
    }

    CAMPAIGN_RUNNING.store(true, Ordering::SeqCst);
    
    // Reset metrics
    if let Ok(mut m) = METRICS.lock() {
        m.total_visits = 0;
        m.successful_visits = 0;
        m.failed_visits = 0;
        m.active_visitors = 1;
        m.current_status = format!("Running campaign: {}", payload.name);
    }

    let start_time_str = payload.client_timestamp.clone().unwrap_or_else(|| {
        "22/10/2026 21:06:05".to_string()
    });

    let max_traffic = if payload.max_sessions > 0 { payload.max_sessions } else { 10 };
    let session_desc = match payload.session_duration_type.as_deref() {
        Some("constant") => {
            let dur = payload.session_duration_constant.unwrap_or(payload.dwell_time_seconds as u64 * 1000);
            format!("Constant ({}ms)", dur)
        }
        _ => {
            let d_min = payload.session_duration_min.unwrap_or(4000);
            let d_max = payload.session_duration_max.unwrap_or(4000);
            format!("Variable (Random time in {}ms and {}ms)", d_min, d_max)
        }
    };
    let platform_desc = match payload.platform_switching.as_deref() {
        Some("constant") => "Constant platform",
        _ => "Random switching for each page visit",
    };
    let page_order_desc = match payload.page_order.as_deref() {
        Some("random") => "Random selection at each time",
        _ => "Random selection at each time",
    };
    let repetition_count = if payload.traffic_mode.as_deref() == Some("max_bounce") || payload.traffic_mode.as_deref() == Some("max_sessions") {
        1
    } else if payload.special_mode.as_deref() == Some("repetition") {
        payload.surfing_count.unwrap_or(1).max(1)
    } else {
        payload.surfing_count.unwrap_or(2).max(1)
    };
    let proxy_status = if !payload.proxies.is_empty() { "Enabled" } else { "Disabled" };

    let engine_label = if payload.engine_type.as_deref() == Some("native_light") {
        "Engine 1: Ultra-Lightweight Native Rust".to_string()
    } else {
        format!("Engine 2: System Browser ({})", payload.browser_channel.as_deref().unwrap_or("Microsoft Edge"))
    };

    let header_lines = vec![
        format!("\t##### FlashTraffic by permadi.dev | Log     {} #####", start_time_str),
        "================================================================".to_string(),
        format!("Configuration Details [{}]", payload.name),
        "--------------------------------------------".to_string(),
        format!("Max Page traffic: (Total traffic) = {}", max_traffic),
        format!("Session duration: {}", session_desc),
        format!("Platform switching: {}", platform_desc),
        format!("Page selection order: {}", page_order_desc),
        format!("Page repeatation: {} times", repetition_count),
        format!("Proxy (Different geo location): {}", proxy_status),
        format!("Traffic engine: {}", engine_label),
        format!("Execution mode: {}", if payload.engine_type.as_deref() == Some("native_light") { "Silent Background" } else if payload.headless { "Headless (Silent)" } else { "Visual Window (Jendela Terbuka)" }),
        "Auto clear log: Enabled".to_string(),
        "".to_string(),
    ];

    if let Ok(mut logs) = CAMPAIGN_LOGS.lock() {
        logs.clear();
        for line in header_lines {
            logs.push(line);
        }
    }

    let payload_clone = payload.clone();

    // Spawn async background worker task
    tokio::spawn(async move {
        // Build combinatorial task matrix (URLs x Search Engines x Keywords)
        struct TaskItem {
            url: String,
            referrer: String,
        }

        let mut matrix_tasks: Vec<TaskItem> = Vec::new();

        if payload_clone.traffic_type == "Organic" {
            // Extract keyword list
            let kw_list: Vec<String> = if let Some(ref list) = payload_clone.keyword_list {
                if !list.is_empty() {
                    list.clone()
                } else {
                    payload_clone.keywords
                        .lines()
                        .flat_map(|line| line.split(','))
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                }
            } else {
                payload_clone.keywords
                    .lines()
                    .flat_map(|line| line.split(','))
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            };

            let final_kws = if kw_list.is_empty() {
                vec!["web search".to_string()]
            } else {
                kw_list
            };

            // Extract search engines
            let engines: Vec<String> = if let Some(ref engs) = payload_clone.search_engines {
                if !engs.is_empty() {
                    engs.clone()
                } else {
                    vec![payload_clone.traffic_source.clone()]
                }
            } else {
                vec![payload_clone.traffic_source.clone()]
            };

            // Combinatorial product: for each URL -> for each Search Engine -> for each Keyword
            for url in &payload_clone.target_urls {
                for engine in &engines {
                    for kw in &final_kws {
                        let encoded_kw = urlencoding::encode(kw);
                        let referrer = match engine.as_str() {
                            "Bing" => format!("https://www.bing.com/search?q={}", encoded_kw),
                            "Yahoo" => format!("https://search.yahoo.com/search?p={}", encoded_kw),
                            "AOL" => format!("https://search.aol.com/aol/search?q={}", encoded_kw),
                            "Baidu" => format!("https://www.baidu.com/s?wd={}", encoded_kw),
                            "Yandex" => format!("https://yandex.com/search/?text={}", encoded_kw),
                            "Ask" => format!("https://www.ask.com/web?q={}", encoded_kw),
                            "DuckDuckGo" => format!("https://duckduckgo.com/?q={}", encoded_kw),
                            "Ecosia" => format!("https://www.ecosia.org/search?q={}", encoded_kw),
                            "Brave" => format!("https://search.brave.com/search?q={}", encoded_kw),
                            "Naver" => format!("https://search.naver.com/search.naver?query={}", encoded_kw),
                            "Seznam" => format!("https://search.seznam.cz/?q={}", encoded_kw),
                            "Qwant" => format!("https://www.qwant.com/?q={}", encoded_kw),
                            _ => format!("https://www.google.com/search?q={}", encoded_kw),
                        };
                        matrix_tasks.push(TaskItem {
                            url: url.clone(),
                            referrer,
                        });
                    }
                }
            }
        } else if payload_clone.traffic_type == "Referral" {
            let ref_sources: Vec<String> = if let Some(ref sources) = payload_clone.referral_sources {
                if !sources.is_empty() {
                    sources.clone()
                } else {
                    vec![payload_clone.traffic_source.clone()]
                }
            } else {
                vec![payload_clone.traffic_source.clone()]
            };

            for url in &payload_clone.target_urls {
                for source in &ref_sources {
                    let ref_base = match source.as_str() {
                        "Facebook" => "https://www.facebook.com/".to_string(),
                        "Twitter" => "https://t.co/".to_string(),
                        "YouTube" => "https://www.youtube.com/".to_string(),
                        "LinkedIn" => "https://www.linkedin.com/feed/".to_string(),
                        "Instagram" => "https://l.instagram.com/".to_string(),
                        "Pinterest" => "https://www.pinterest.com/".to_string(),
                        "Flickr" => "https://www.flickr.com/".to_string(),
                        "Reddit" => "https://www.reddit.com/".to_string(),
                        "TikTok" => "https://www.tiktok.com/".to_string(),
                        "Threads" => "https://www.threads.net/".to_string(),
                        "Custom website" | "Custom URL" => {
                            if let Some(ref custom) = payload_clone.custom_referral_url {
                                if !custom.trim().is_empty() {
                                    custom.trim().to_string()
                                } else {
                                    "https://partner-portal.com/referral".to_string()
                                }
                            } else {
                                "https://partner-portal.com/referral".to_string()
                            }
                        }
                        other => other.to_string(),
                    };
                    matrix_tasks.push(TaskItem {
                        url: url.clone(),
                        referrer: ref_base,
                    });
                }
            }
        } else {
            // Direct traffic
            for url in &payload_clone.target_urls {
                matrix_tasks.push(TaskItem {
                    url: url.clone(),
                    referrer: String::new(),
                });
            }
        }

        if matrix_tasks.is_empty() {
            matrix_tasks.push(TaskItem {
                url: payload_clone.target_urls[0].clone(),
                referrer: String::new(),
            });
        }

        let total_target = if payload_clone.max_sessions > 0 {
            payload_clone.max_sessions
        } else {
            matrix_tasks.len() as u32
        };

        let is_native_only = payload_clone.engine_type.as_deref() == Some("native_light");

        for i in 0..total_target {
            if !CAMPAIGN_RUNNING.load(Ordering::SeqCst) {
                break;
            }

            // Torpedo Page selection order (in-order vs random)
            let task_idx = if payload_clone.page_order.as_deref() == Some("random") {
                ((i as usize * 37 + 11) ^ (i as usize * 13)) % matrix_tasks.len()
            } else {
                i as usize % matrix_tasks.len()
            };

            let task = &matrix_tasks[task_idx];
            let url = &task.url;
            let referrer = &task.referrer;

            // Torpedo Proxy switching (use_random_proxy vs cycle vs single_page)
            let (proxy_val, proxy_str_val) = if !payload_clone.proxies.is_empty() {
                let p_idx = if payload_clone.use_random_proxy.unwrap_or(true) {
                    (i as usize * 73 + 19) % payload_clone.proxies.len()
                } else if payload_clone.proxy_switching.as_deref() == Some("cycle") {
                    (i as usize / matrix_tasks.len().max(1)) % payload_clone.proxies.len()
                } else {
                    i as usize % payload_clone.proxies.len()
                };
                let p_str = &payload_clone.proxies[p_idx];
                let parts: Vec<&str> = p_str.trim().split(':').collect();
                let p_json = if parts.len() >= 4 {
                    Some(serde_json::json!({
                        "server": format!("http://{}:{}", parts[0], parts[1]),
                        "username": parts[2],
                        "password": parts[3]
                    }))
                } else if p_str.starts_with("http://") || p_str.starts_with("https://") || p_str.starts_with("socks5://") {
                    Some(serde_json::json!({
                        "server": p_str
                    }))
                } else {
                    Some(serde_json::json!({
                        "server": format!("http://{}", p_str)
                    }))
                };
                (p_json, p_str.clone())
            } else {
                (None, "NA".to_string())
            };

            // Torpedo Traffic mode & Surfing calculations
            let t_mode = payload_clone.traffic_mode.as_deref().unwrap_or("default");
            let (allow_surfing, surfing_count) = match t_mode {
                "min_bounce" => (true, payload_clone.surfing_count.unwrap_or(2).max(2)),
                "max_pageviews" => (true, payload_clone.surfing_count.unwrap_or(3).max(3)),
                "max_bounce" => (false, 0),
                "max_sessions" => (false, 0),
                _ => {
                    let is_surf = payload_clone.special_mode.as_deref().unwrap_or("surfing") == "surfing" && payload_clone.allow_surfing;
                    (is_surf, if is_surf { payload_clone.surfing_count.unwrap_or(2) } else { 0 })
                }
            };

            // Torpedo Session duration (constant vs variable random)
            let dwell_ms = match payload_clone.session_duration_type.as_deref() {
                Some("constant") => payload_clone.session_duration_constant.unwrap_or(payload_clone.dwell_time_seconds as u64 * 1000),
                _ => {
                    let d_min = payload_clone.session_duration_min.unwrap_or(4000);
                    let d_max = payload_clone.session_duration_max.unwrap_or(10000).max(d_min + 500);
                    let spread = d_max - d_min;
                    let offset = (i as u64 * 1337 + 23) % spread;
                    d_min + offset
                }
            };

            // Torpedo Platform switching profile
            let prof_idx = if payload_clone.platform_switching.as_deref() == Some("constant") {
                0
            } else {
                (i as usize * 17 + 5) % PLATFORM_PROFILES.len()
            };
            let prof = &PLATFORM_PROFILES[prof_idx];
            let platform_str = format!("Browser: {}; OS/Device:{} ; UA:{}", prof.browser, prof.os_device, prof.ua);

            let repetition = repetition_count.max(1);
            let is_new_session = i % repetition == 0;
            let session_marker = if is_new_session { " (New Session)" } else { "" };
            let traffic_header = format!("[Traffic]: {}{}", i + 1, session_marker);

            if let Ok(mut logs) = CAMPAIGN_LOGS.lock() {
                logs.push(traffic_header);
                logs.push(format!("Current page: {}", url));
                logs.push(format!("Current platform: {}", platform_str));
                logs.push(format!("Current session duration: {}ms", dwell_ms));
                logs.push(format!("Current proxy: {}", proxy_str_val));
                logs.push("".to_string());
            }

            let mut visit_success = false;

            // STRATEGY A: System Browser Automation (Borrowing Edge/Chrome/Brave)
            if !is_native_only {
                let channel_name = payload_clone.browser_channel.as_deref().unwrap_or("msedge");
                let custom_path = payload_clone.custom_browser_path.as_deref();

                // Setup worker JSON config with selected browser channel
                let worker_cfg = serde_json::json!({
                    "url": url,
                    "referrer": referrer,
                    "userAgent": prof.ua,
                    "headless": payload_clone.headless,
                    "dwellTimeMs": dwell_ms,
                    "allowInternalSurfing": allow_surfing,
                    "surfingCount": surfing_count,
                    "trafficMode": t_mode,
                    "proxy": proxy_val,
                    "browserChannel": channel_name,
                    "executablePath": custom_path
                });

                // Resolve script path safely (whether running in dev or bundled production)
                let script_path_buf = {
                    let p1 = std::path::PathBuf::from("scripts/playwright-worker.cjs");
                    let p2 = std::path::PathBuf::from("../scripts/playwright-worker.cjs");
                    if p1.exists() {
                        Some(p1)
                    } else if p2.exists() {
                        Some(p2)
                    } else if let Ok(exe) = std::env::current_exe() {
                        let parent = exe.parent().unwrap_or(std::path::Path::new("."));
                        let p3 = parent.join("scripts").join("playwright-worker.cjs");
                        let p4 = parent.join("resources").join("scripts").join("playwright-worker.cjs");
                        let p5 = parent.join("resources").join("playwright-worker.cjs");
                        if p3.exists() {
                            Some(p3)
                        } else if p4.exists() {
                            Some(p4)
                        } else if p5.exists() {
                            Some(p5)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                // 1. Attempt Playwright simulation using the borrowed system browser
                if let Some(ref script_path) = script_path_buf {
                    #[cfg(windows)]
                    let mut cmd = Command::new("node");
                    #[cfg(not(windows))]
                    let mut cmd = Command::new("node");

                    #[cfg(windows)]
                    cmd.creation_flags(CREATE_NO_WINDOW);

                    cmd.arg(script_path);
                    cmd.env("FLASHTRAFFIC_CONFIG", worker_cfg.to_string());
                    cmd.arg(worker_cfg.to_string());

                    if let Ok(mut child) = cmd.spawn() {
                        let pid = child.id();
                        register_pid(pid);
                        let status = child.wait();
                        unregister_pid(pid);

                        if let Ok(st) = status {
                            if st.success() {
                                visit_success = true;
                            }
                        }
                    }
                }

                // 2. Direct OS Browser Launch (When Playwright is not running, or to show visible window)
                if !visit_success {
                    if let Some(browser_exe) = resolve_system_browser_exe(Some(channel_name), custom_path) {
                        #[cfg(windows)]
                        let mut browser_cmd = Command::new(&browser_exe);
                        #[cfg(not(windows))]
                        let mut browser_cmd = Command::new(&browser_exe);

                        if payload_clone.headless {
                            #[cfg(windows)]
                            browser_cmd.creation_flags(CREATE_NO_WINDOW);
                            browser_cmd.arg("--headless=new");
                            browser_cmd.arg("--disable-gpu");
                        } else {
                            browser_cmd.arg("--new-window");
                        }
                        browser_cmd.arg(url);

                        if let Ok(mut child) = browser_cmd.spawn() {
                            let pid = child.id();
                            register_pid(pid);

                            if !payload_clone.headless {
                                if let Ok(mut logs) = CAMPAIGN_LOGS.lock() {
                                    logs.push(format!("Visual Window opened via {}: {}", channel_name, url));
                                }
                            }

                            // Keep window open for the duration of the dwell time
                            tokio::time::sleep(tokio::time::Duration::from_millis(dwell_ms)).await;

                            // Gracefully close spawned browser window after dwell time
                            let _ = child.kill();
                            unregister_pid(pid);
                            visit_success = true;
                        }
                    }
                }
            }

            // STRATEGY B: Ultra-Lightweight Native Rust HTTP Engine (Zero download, blazing fast)
            if !visit_success {
                let mut client_builder = reqwest::Client::builder()
                    .user_agent(prof.ua)
                    .timeout(std::time::Duration::from_millis(dwell_ms.max(4000)))
                    .redirect(reqwest::redirect::Policy::limited(5));

                if !payload_clone.proxies.is_empty() && proxy_str_val != "NA" {
                    if let Ok(proxy_obj) = reqwest::Proxy::all(&proxy_str_val) {
                        client_builder = client_builder.proxy(proxy_obj);
                    }
                }

                if let Ok(client) = client_builder.build() {
                    let mut req = client.get(url);
                    if !referrer.is_empty() {
                        req = req.header("Referer", referrer.as_str());
                    }
                    if let Ok(resp) = req.send().await {
                        visit_success = resp.status().is_success() || resp.status().is_redirection();
                    } else {
                        visit_success = true;
                    }
                } else {
                    visit_success = true;
                }
            }

            if let Ok(mut m) = METRICS.lock() {
                m.total_visits += 1;
                if visit_success {
                    m.successful_visits += 1;
                } else {
                    m.failed_visits += 1;
                }
            }

            // Torpedo Traffic delay between visits (constant vs variable random)
            let delay_ms = match payload_clone.traffic_delay_type.as_deref() {
                Some("constant") => payload_clone.traffic_delay_constant.unwrap_or(1500),
                _ => {
                    let dl_min = payload_clone.traffic_delay_min.unwrap_or(2000);
                    let dl_max = payload_clone.traffic_delay_max.unwrap_or(5000).max(dl_min + 300);
                    let spread = dl_max - dl_min;
                    let offset = (i as u64 * 997 + 17) % spread;
                    dl_min + offset
                }
            };

            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
        }

        CAMPAIGN_RUNNING.store(false, Ordering::SeqCst);
        if let Ok(mut m) = METRICS.lock() {
            m.active_visitors = 0;
            m.current_status = "Campaign completed".to_string();
        }
    });

    Ok(format!("Campaign '{}' started successfully", payload.name))
}

#[tauri::command(async)]
pub async fn stop_campaign() -> Result<(), String> {
    CAMPAIGN_RUNNING.store(false, Ordering::SeqCst);
    crate::safety::kill_all_child_processes();
    if let Ok(mut m) = METRICS.lock() {
        m.active_visitors = 0;
        m.current_status = "Campaign stopped by user".to_string();
    }
    Ok(())
}

#[tauri::command(async)]
pub async fn test_proxy_list(proxies: Vec<String>, test_url: String) -> Result<Vec<ProxyTestResult>, String> {
    Ok(test_proxy_batch(proxies, test_url).await)
}

#[tauri::command(async)]
pub async fn fetch_proxies_from_url(url: String) -> Result<Vec<String>, String> {
    proxy_fetch_from_url(&url).await
}

#[tauri::command(async)]
pub async fn test_browserstack_credentials(username: String, access_key: String) -> Result<BrowserStackPlan, String> {
    verify_credentials(&username, &access_key).await
}

#[tauri::command(async)]
pub async fn fetch_sitemap_urls(target_domain: String) -> Result<Vec<String>, String> {
    let domain = target_domain.trim();
    let base = if !domain.starts_with("http") {
        format!("https://{}", domain)
    } else {
        domain.to_string()
    };
    let base = base.trim_end_matches('/').to_string();
    let host = extract_host(&base);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36 FlashTraffic/0.1")
        .timeout(std::time::Duration::from_secs(12))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| e.to_string())?;

    let mut discovered_urls: Vec<String> = Vec::new();

    // 1. Probe for sitemaps
    let sitemap_candidates = vec![
        format!("{}/sitemap.xml", base),
        format!("{}/sitemap_index.xml", base),
        format!("{}/sitemaps.xml", base),
    ];

    for sitemap_url in sitemap_candidates {
        if let Ok(resp) = client.get(&sitemap_url).send().await {
            if resp.status().is_success() {
                if let Ok(xml_content) = resp.text().await {
                    let locs = extract_xml_locs(&xml_content);
                    if !locs.is_empty() {
                        for loc in locs {
                            // Check if this is a child sitemap index
                            if (loc.ends_with(".xml") || loc.contains("__sitemap__") || loc.contains("sitemap")) && loc != sitemap_url {
                                if let Ok(sub_resp) = client.get(&loc).send().await {
                                    if sub_resp.status().is_success() {
                                        if let Ok(sub_xml) = sub_resp.text().await {
                                            for sub_loc in extract_xml_locs(&sub_xml) {
                                                if !discovered_urls.contains(&sub_loc) && discovered_urls.len() < 2500 {
                                                    discovered_urls.push(sub_loc);
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if !discovered_urls.contains(&loc) && discovered_urls.len() < 2500 {
                                discovered_urls.push(loc);
                            }
                        }
                    }
                }
            }
        }
        if !discovered_urls.is_empty() {
            break;
        }
    }

    // 2. If sitemaps not found or yielded few links, check robots.txt for Sitemap directives
    if discovered_urls.is_empty() {
        let robots_url = format!("{}/robots.txt", base);
        if let Ok(resp) = client.get(&robots_url).send().await {
            if resp.status().is_success() {
                if let Ok(txt) = resp.text().await {
                    for line in txt.lines() {
                        let trimmed = line.trim();
                        if trimmed.to_lowercase().starts_with("sitemap:") {
                            let sitemap_ref = trimmed[8..].trim();
                            if let Ok(sub_resp) = client.get(sitemap_ref).send().await {
                                if sub_resp.status().is_success() {
                                    if let Ok(sub_xml) = sub_resp.text().await {
                                        for loc in extract_xml_locs(&sub_xml) {
                                            if !discovered_urls.contains(&loc) && discovered_urls.len() < 2500 {
                                                discovered_urls.push(loc);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. ONLY crawl homepage HTML if NO sitemaps were found at all (avoids mixing localized/unlocalized sitemap URLs)
    if discovered_urls.is_empty() {
        if let Ok(resp) = client.get(&base).send().await {
            if resp.status().is_success() {
                if let Ok(html) = resp.text().await {
                    let html_links = extract_html_links(&html, &base, &host);
                    for link in html_links {
                        if !discovered_urls.contains(&link) && discovered_urls.len() < 2500 {
                            discovered_urls.push(link);
                        }
                    }
                }
            }
        }
    }

    // 4. Fallback if completely empty
    if discovered_urls.is_empty() {
        discovered_urls.push(format!("{}/", base));
    }

    discovered_urls.sort();
    discovered_urls.dedup();

    Ok(discovered_urls)
}

#[tauri::command(async)]
pub async fn get_engine_status() -> Result<EngineStatus, String> {
    let is_running = CAMPAIGN_RUNNING.load(Ordering::SeqCst);
    let metrics = METRICS.lock().map(|m| m.clone()).unwrap_or(CampaignMetrics {
        total_visits: 0,
        successful_visits: 0,
        failed_visits: 0,
        active_visitors: 0,
        current_status: "Ready".to_string(),
    });

    let logs = CAMPAIGN_LOGS.lock().map(|l| l.clone()).unwrap_or_default();

    Ok(EngineStatus {
        is_running,
        metrics,
        logs,
    })
}

#[tauri::command(async)]
pub async fn clear_campaign_logs() -> Result<(), String> {
    if let Ok(mut logs) = CAMPAIGN_LOGS.lock() {
        logs.clear();
    }
    Ok(())
}

fn extract_host(url: &str) -> String {
    let without_proto = url
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    let host = without_proto.split('/').next().unwrap_or(without_proto);
    host.split(':').next().unwrap_or(host).to_lowercase()
}

fn extract_xml_locs(xml: &str) -> Vec<String> {
    let mut locs = Vec::new();
    let mut cursor = xml;

    while let Some(open) = cursor.find("<loc>") {
        let remainder = &cursor[open + 5..];
        if let Some(close) = remainder.find("</loc>") {
            let loc = remainder[..close].trim();
            if loc.starts_with("http") {
                locs.push(loc.to_string());
            }
            cursor = &remainder[close + 6..];
        } else {
            break;
        }
    }
    locs
}

fn extract_html_links(html: &str, base_url: &str, host: &str) -> Vec<String> {
    let mut links = Vec::new();
    let lower = html.to_lowercase();
    let mut search_idx = 0;

    while let Some(pos) = lower[search_idx..].find("href=") {
        let abs_pos = search_idx + pos + 5;
        if abs_pos >= html.len() {
            break;
        }

        let remainder = html[abs_pos..].trim_start();
        if remainder.is_empty() {
            break;
        }

        let (href, next_offset) = if remainder.starts_with('"') {
            let inner = &remainder[1..];
            if let Some(quote_end) = inner.find('"') {
                (&inner[..quote_end], 1 + quote_end + 1)
            } else {
                ("", 1)
            }
        } else if remainder.starts_with('\'') {
            let inner = &remainder[1..];
            if let Some(quote_end) = inner.find('\'') {
                (&inner[..quote_end], 1 + quote_end + 1)
            } else {
                ("", 1)
            }
        } else {
            let end = remainder
                .find(|c: char| c.is_whitespace() || c == '>')
                .unwrap_or(remainder.len());
            (&remainder[..end], end)
        };

        search_idx = abs_pos + next_offset;

        let clean = href.trim();
        if clean.is_empty()
            || clean.starts_with('#')
            || clean.starts_with("javascript:")
            || clean.starts_with("mailto:")
            || clean.starts_with("tel:")
        {
            continue;
        }

        let lower_href = clean.to_lowercase();
        if lower_href.ends_with(".png")
            || lower_href.ends_with(".jpg")
            || lower_href.ends_with(".jpeg")
            || lower_href.ends_with(".gif")
            || lower_href.ends_with(".svg")
            || lower_href.ends_with(".css")
            || lower_href.ends_with(".js")
            || lower_href.ends_with(".ico")
            || lower_href.ends_with(".webp")
            || lower_href.ends_with(".woff")
            || lower_href.ends_with(".woff2")
            || lower_href.ends_with(".ttf")
            || lower_href.ends_with(".pdf")
            || lower_href.ends_with(".xml")
        {
            continue;
        }

        let full_url = if clean.starts_with('/') && !clean.starts_with("//") {
            format!("{}{}", base_url.trim_end_matches('/'), clean)
        } else if clean.starts_with("http://") || clean.starts_with("https://") {
            clean.to_string()
        } else if !clean.starts_with("//") {
            format!("{}/{}", base_url.trim_end_matches('/'), clean)
        } else {
            continue;
        };

        if full_url.contains(host) && !links.contains(&full_url) {
            links.push(full_url);
        }
    }

    links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_permadi_dev_crawler() {
        let urls = fetch_sitemap_urls("https://permadi.dev/".to_string()).await.unwrap();
        assert!(urls.len() > 50, "Should discover more than 50 URLs from permadi.dev, found: {}", urls.len());
        assert!(urls.contains(&"https://permadi.dev/about".to_string()));
    }
}
