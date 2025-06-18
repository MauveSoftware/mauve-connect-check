mod check;
mod check_result;
mod cli;
mod output;

use std::process::exit;

use clap::Parser;

use crate::check::check_mauve_dns;
use crate::cli::Cli;
use crate::output::print_check_result;

enum ExitCode {
    CheckPassed = 0,
    CheckError = 1,
    CheckFailed = 2,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let domain = &args.domain;
    match check_mauve_dns(domain).await {
        Ok(result) => {
            print_check_result(domain, &result, args.verbose);
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
