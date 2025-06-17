mod check_result;
mod output;

use crate::check_result::CheckResult;
use crate::output::print_check_result;

use std::env;
use std::error::Error;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <domain>", args[0]);
        exit(1);
    }
    let domain = &args[1];

    match check_mauve_dns(domain).await {
        Ok(result) => {
            print_check_result(domain, &result);
            if result.success {
                exit(0);
            } else {
                exit(-1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);
        }
    }
}

async fn check_mauve_dns(domain: &String) -> Result<CheckResult, Box<dyn Error>> {
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
