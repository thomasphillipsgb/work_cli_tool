use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Work CLI Tool", version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    command: Option<String>,
}