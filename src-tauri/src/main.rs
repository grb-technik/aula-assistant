// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aula_assistant_lib::{config::RuntimeConfigBuilder, run};
use cli::{Mode, parse_args, print_license, print_version};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = parse_args()?;

    match mode {
        Mode::ShowLicense => {
            print_license();
            return Ok(());
        }
        Mode::ShowVersion => {
            let commit_id = std::env::var("LAST_COMMIT_ID").ok();
            let commit_date = std::env::var("LAST_COMMIT_DATE").ok();
            print_version(&commit_id, &commit_date);
            return Ok(());
        }
        Mode::Run(args) => {
            let config = RuntimeConfigBuilder::new()
                .tablet_mode(args.tablet_mode())
                .fullscreen(args.fullscreen())
                .config_file(args.config_file().cloned())
                .build();

            if let Err(err) = run(config) {
                eprintln!("Application error: {err:?}");
                std::process::exit(1);
            }

            Ok(())
        }
    }
}
