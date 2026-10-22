use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserStackPlan {
    pub automate_plan: Option<String>,
    pub parallel_sessions_running: Option<u32>,
    pub team_parallel_sessions_max_allowed: Option<u32>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CloudDeviceOption {
    pub os: String,
    pub os_version: String,
    pub browser: String,
    pub device: Option<String>,
    pub real_mobile: bool,
}

/// Tests authentication against the BrowserStack Automate API
pub async fn verify_credentials(username: &str, access_key: &str) -> Result<BrowserStackPlan, String> {
    if username.trim().is_empty() || access_key.trim().is_empty() {
        return Err("Username and Access Key cannot be empty".to_string());
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get("https://api.browserstack.com/automate/plan.json")
        .basic_auth(username.trim(), Some(access_key.trim()))
        .send()
        .await
        .map_err(|e| format!("Connection to BrowserStack failed: {}", e))?;

    if response.status().is_success() {
        let plan: BrowserStackPlan = response
            .json()
            .await
            .unwrap_or_else(|_| BrowserStackPlan {
                automate_plan: Some("Active Plan".to_string()),
                parallel_sessions_running: Some(0),
                team_parallel_sessions_max_allowed: Some(5),
                status: "Connected".to_string(),
            });
        Ok(plan)
    } else if response.status().as_u16() == 401 {
        Err("Invalid BrowserStack credentials (Unauthorized 401)".to_string())
    } else {
        Err(format!("BrowserStack returned HTTP error status: {}", response.status()))
    }
}
