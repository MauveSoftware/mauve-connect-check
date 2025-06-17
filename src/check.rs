use anyhow::{Context, Result};
use std::error::Error;

use crate::check_result::CheckResult;

const BASE_URL: &str = "https://domain-manager.infra.mauve.cloud/api/dns/check/";

pub async fn check_mauve_dns(domain: &String) -> Result<CheckResult, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, domain);
    let resp = reqwest::get(url).await.context("unable to reach service")?;
    if resp.status() != reqwest::StatusCode::OK {
        let mut err_msg = format!("HTTP error: {}", resp.status());

        if let Ok(body) = resp.text().await {
            err_msg += &format!("\n{}", body);
        }
        return Err(err_msg.into());
    }

    let res = resp
        .json::<CheckResult>()
        .await
        .context("coult not parse check result")?;
    Ok(res)
}
