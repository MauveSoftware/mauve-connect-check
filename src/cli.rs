use clap::Parser;

#[derive(Parser)]
#[command(
    version,
    name = "mauve-connect-check",
    author = "Daniel Brendgen-Czerwonk",
    about = "Simple CLI tool to check for DNS configuratation issues"
)]
pub struct Cli {
    /// The domain to process
    #[arg(required = true, index = 1)]
    pub domain: String,

    /// Enable verbose output
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,
}
