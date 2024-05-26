use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "fish-lsp",
    version = "1.0",
    about = "Fish Language Server Protocol"
)]
pub struct Cli {
    #[arg(short, long, default_value = "info")]
    pub log_level: String,
}
