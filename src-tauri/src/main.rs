// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aula_assistant_lib::{config::RuntimeConfigBuilder, run};
use clap::{ArgAction, arg, command, crate_name, crate_version, value_parser};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        // help
        .after_help("Please refer to the documentation for further information.")
        .after_long_help("Please refer to the documentation for further information.")
        .next_line_help(true)
        // version
        .disable_version_flag(true)
        .arg(
            arg!(-v --version "Print version")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        // license
        .arg(
            arg!(--license "Print license")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        // tablet mode
        .arg(
            arg!(-t --"tablet" "Enable tablet mode")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        // open in fullscreen
        .arg(
            arg!(-f --"fullscreen" "Open in fullscreen")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        // config file path
        .arg(
            arg!(-c --"config-file" <FILE> "Path to a configuration file")
                .value_parser(value_parser!(std::path::PathBuf))
                .action(ArgAction::Set),
        )
        .get_matches();

    if matches.get_flag("version") {
        print_version();
        return Ok(());
    }

    if matches.get_flag("license") {
        println!(include_str!("../../LICENSE.txt"));
        return Ok(());
    }

    let config = RuntimeConfigBuilder::new()
        .tablet_mode(matches.get_flag("tablet"))
        .fullscreen(matches.get_flag("fullscreen"))
        .config_file(matches.get_one::<PathBuf>("config-file").cloned())
        .build();

    if let Err(err) = run(config) {
        eprintln!("Application error: {err:?}");
        std::process::exit(1);
    }

    Ok(())
}

fn print_version() {
    let commit_id = std::env::var("LAST_COMMIT_ID").ok();
    let commit_date = std::env::var("LAST_COMMIT_DATE").ok();

    match (commit_id, commit_date) {
        (Some(id), Some(date)) => {
            println!("{} {} ({} {})", crate_name!(), crate_version!(), id, date,);
        }
        _ => {
            println!("{} {}", crate_name!(), crate_version!());
        }
    }
}
