// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{ArgAction, arg, command, crate_name, crate_version};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        // help
        .after_help("Please refer to the documentation for further information.")
        .after_long_help("Please refer to the documentation for further information.")
        .next_line_help(true)
        // version
        .disable_version_flag(true)
        .arg(arg!(-v --version "Print version").action(ArgAction::SetTrue))
        // license
        .arg(arg!(--license "Print license").action(ArgAction::SetTrue))
        // tablet mode
        .arg(arg!(-t --"tablet" "Tablet mode").action(ArgAction::SetTrue))
        // config file path
        .arg(
            arg!(-c --"config-file" <FILE> "Path to the configuration file")
                .value_name("FILE")
                .required(false),
        )
        .get_matches();

    if matches.get_flag("version") {
        let commit_id = std::env::var("LAST_COMMIT_ID").ok();
        let commit_date = std::env::var("LAST_COMMIT_DATE").ok();

        if commit_id.is_none() || commit_date.is_none() {
            println!("{} {}", crate_name!(), crate_version!());
            return Ok(());
        }

        println!(
            "{} {} ({} {})",
            crate_name!(),
            crate_version!(),
            commit_id.unwrap(),
            commit_date.unwrap(),
        );

        return Ok(());
    }

    if matches.get_flag("license") {
        println!(include_str!("../../LICENSE.txt"));
        return Ok(());
    }

    let mut cb = aula_assistant_lib::config::RuntimeConfigBuilder::new();

    if matches.get_flag("tablet") {
        cb = cb.tablet_mode(true);
    }

    if let Some(config_file) = matches.get_one::<String>("config-file") {
        cb = cb.config_file(config_file.to_string());
    }

    aula_assistant_lib::run(cb.build()).expect("error while running tauri application");

    Ok(())
}
