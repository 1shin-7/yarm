#![windows_subsystem = "windows"]

mod cli;
mod display;
mod ui;
mod utils;

use clap::Parser;
use log::error;

fn main() -> anyhow::Result<()> {
    // On Windows, attempt to attach to the parent process's console.
    // This allows stdout/stderr to work if run from a terminal (CLI mode),
    // while ensuring no console window appears if run from GUI (Explorer).
    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }

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
