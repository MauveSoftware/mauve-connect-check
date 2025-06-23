mod check;
mod check_result;
mod cli;
mod output;

use std::process::ExitCode;

use clap::Parser;

use crate::check::check_mauve_dns;
use crate::cli::Cli;
use crate::output::print_check_result;

enum CheckResult {
    Passed,
    Error,
    Failed,
}

impl CheckResult {
    fn exit_code(&self) -> ExitCode {
        match self {
            CheckResult::Passed => ExitCode::from(0),
            CheckResult::Error => ExitCode::from(1),
            CheckResult::Failed => ExitCode::from(3),
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Cli::parse();

    let domain = &args.domain;
    match check_mauve_dns(domain).await {
        Ok(result) => {
            print_check_result(domain, &result, args.verbose);
            if result.success {
                CheckResult::Passed.exit_code()
            } else {
                CheckResult::Failed.exit_code()
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            CheckResult::Error.exit_code()
        }
    }
}
