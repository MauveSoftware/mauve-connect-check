mod check;
mod check_result;
mod output;

use std::env;
use std::process::exit;

use crate::check::check_mauve_dns;
use crate::output::print_check_result;

enum ExitCode {
    MissingArgument = 1,
    CheckError = 2,
    CheckPassed = 0,
    CheckFailed = -1,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <domain>", args[0]);
        exit(ExitCode::MissingArgument as i32);
    }
    let domain = &args[1];

    match check_mauve_dns(domain).await {
        Ok(result) => {
            print_check_result(domain, &result);
            if result.success {
                exit(ExitCode::CheckPassed as i32);
            } else {
                exit(ExitCode::CheckFailed as i32);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(ExitCode::CheckError as i32);
        }
    }
}
