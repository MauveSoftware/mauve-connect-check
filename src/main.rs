mod check_result;

use check_result::{CheckResult, Record};
use colored::Colorize;
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

fn print_check_result(domain: &String, result: &CheckResult) {
    println!("Domain: {}", domain.cyan());
    print!("Status: ");

    if result.success {
        println!("{}", "success".green());
    } else {
        println!("{}", "fail".red());
    }

    result.records.iter().for_each(|record| {
        println!();
        print_record(record);
    });
}

fn print_record(record: &Record) {
    println!("{} {}", record.type_name.bold(), record.name.cyan());
    print!("Status:");

    if record.passed {
        println!(" {}", "passed".green());
    } else {
        println!(" {}", "failed".red());
    }

    if record.incomplete {
        println!("{}", "incomplete".yellow());
    }

    if record.passed && record.legacy {
        println!("{}", "deprecated: use CNAME instead".yellow());
    }

    print_diff(record);
}

fn print_diff(record: &Record) {
    if record.legacy && record.type_name == "CNAME" {
        return;
    }

    println!("Values:");
    let mut actual: Vec<String> = record.actual.iter().map(|s| s.to_string()).collect();
    record.expected.iter().for_each(|exp_val| {
        let find_res = actual.iter().position(|a| is_matching_value(exp_val, a));
        match find_res {
            Some(idx) => {
                println!("  {}: {}", exp_val.green(), "ok");
                actual.remove(idx);
            }
            None if record.type_name == "AAAA" => {
                println!("  {}: {}", exp_val.magenta(), "not found");
            }
            None => {
                println!("  {}: {}", exp_val.red(), "not found");
            }
        }
    });

    for a in actual {
        println!("  {}: {}", a.red(), "unexpected");
    }
}

fn is_matching_value(expected: &String, actual: &String) -> bool {
    if actual.starts_with("*") {
        return false;
    }

    if expected.contains('*') {
        let expected_parts: Vec<&str> = expected.split('*').collect();
        return actual.ends_with(expected_parts[1]);
    }

    expected == actual
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_matching_value() {
        assert!(is_matching_value(
            &"*.example.com".to_string(),
            &"sub.example.com".to_string()
        ));
        assert!(!is_matching_value(
            &"*.example.com".to_string(),
            &"example.com".to_string()
        ));
        assert!(is_matching_value(
            &"exact.com".to_string(),
            &"exact.com".to_string()
        ));
        assert!(!is_matching_value(
            &"exact.com".to_string(),
            &"different.com".to_string()
        ));
        assert!(!is_matching_value(
            &"*.example.com".to_string(),
            &"*.example.com".to_string()
        ));
        assert!(is_matching_value(
            &"*.example.com".to_string(),
            &"foo.bar.example.com".to_string()
        ));
    }
}
