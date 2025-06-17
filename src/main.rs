mod check;
mod check_result;
mod output;

use std::env;
use std::process::exit;

use crate::check::check_mauve_dns;
use crate::output::print_check_result;

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
