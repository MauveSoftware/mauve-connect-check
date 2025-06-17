use std::error::Error;

use crate::check_result::CheckResult;

pub async fn check_mauve_dns(domain: &String) -> Result<CheckResult, Box<dyn Error>> {
    let url = format!(
        "https://domain-manager.infra.mauve.cloud/api/dns/check/{}",
        domain
    );
    let resp = reqwest::get(url).await?;
    if resp.status() != reqwest::StatusCode::OK {
        let mut err_msg = format!("HTTP error: {}", resp.status());

        if let Some(body) = resp.text().await.ok() {
            err_msg += &format!("\n{}", body);
        }
        return Err(err_msg.into());
    }

    let res = resp.json::<CheckResult>().await?;
    Ok(res)
}
