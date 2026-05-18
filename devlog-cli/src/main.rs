mod cmd;
mod context;
mod llm;

use clap::{Parser, Subcommand};

use crate::cmd::{setup::handle_setup, standup::run_standup};

#[derive(Parser)]
#[command(name = "devlog")]
#[command(about = "A sleek developer log tool to track daily progress.", long_about = None)]
#[command(arg_required_else_help = true)] // Still forces help if run empty
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setup {
        /// Path to the project directory to monitor
        #[arg(short, long)]
        project: Option<String>,
    },
    Standup,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Setup { project } => {
            handle_setup(project.clone())?;
        }
        Commands::Standup => {
            run_standup().await?;
        }
    }

    Ok(())
}
