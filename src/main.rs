mod cli;
mod display;
mod ui;
mod utils;

use clap::Parser;
use log::error;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = cli::Cli::parse();

    if args.command.is_some() {
        // CLI mode
        if let Err(e) = cli::handle_cli(args) {
            error!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        // GUI mode
        if let Err(e) = ui::run(args.debug) {
            error!("GUI Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
