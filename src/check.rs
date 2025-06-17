use anyhow::{Context, Result};
use std::time::Duration;

use crate::check_result::CheckResult;

const BASE_URL: &str = "https://domain-manager.infra.mauve.cloud/api/dns/check/";
const TIMEOUT_SECONDS: u64 = 10;

pub async fn check_mauve_dns(domain: &str) -> Result<CheckResult> {
    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(TIMEOUT_SECONDS))
        .build()
        .context("could not build HTTP client")?;

    let url = format!("{}{}", BASE_URL, domain);
    let resp = client
        .get(url)
        .send()
        .await
        .context("unable to reach service")?;
    if resp.status() != reqwest::StatusCode::OK {
        let mut err_msg = format!("HTTP error: {}", resp.status());

        if let Ok(body) = resp.text().await {
            err_msg += &format!("\n{}", body);
        }
        return Err(anyhow::anyhow!(err_msg).context("service returned an error"));
    }

    let res = resp
        .json::<CheckResult>()
        .await
        .context("could not parse check result")?;
    Ok(res)
}
