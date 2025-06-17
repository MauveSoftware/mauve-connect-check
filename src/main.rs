mod check;
mod check_result;
mod output;

use std::process::exit;

use clap::{Arg, Command};

use crate::check::check_mauve_dns;
use crate::output::print_check_result;

enum ExitCode {
    CheckPassed = 0,
    CheckError = 1,
    CheckFailed = 2,
}

#[tokio::main]
async fn main() {
    let matches = Command::new("mauve-connect-check")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Simple CLI tool to check for DNS configuratation issues")
        .arg(
            Arg::new("domain")
                .help("The domain to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let domain = matches
        .get_one::<String>("domain")
        .expect("domain is required");
    let verbose = matches.get_flag("verbose");

    match check_mauve_dns(domain).await {
        Ok(result) => {
            print_check_result(domain, &result, verbose);
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
