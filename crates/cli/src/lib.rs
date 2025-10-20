use clap::{ArgAction, arg, command, crate_name, crate_version, value_parser};
use std::path::PathBuf;

pub enum Mode {
    ShowLicense,
    ShowVersion,
    Run(Args),
}

pub struct Args {
    tablet_mode: bool,
    fullscreen: bool,
    config_file: Option<PathBuf>,
}

impl Args {
    pub fn tablet_mode(&self) -> bool {
        self.tablet_mode
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn config_file(&self) -> Option<&PathBuf> {
        self.config_file.as_ref()
    }
}

/// will parse command line arguments and return the selected mode
/// except for help which will print help and exit the program automatically
pub fn parse_args() -> Result<Mode, Box<dyn std::error::Error>> {
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
        return Ok(Mode::ShowVersion);
    }

    if matches.get_flag("license") {
        return Ok(Mode::ShowLicense);
    }

    let args = Args {
        tablet_mode: matches.get_flag("tablet"),
        fullscreen: matches.get_flag("fullscreen"),
        config_file: matches.get_one::<PathBuf>("config-file").cloned(),
    };

    Ok(Mode::Run(args))
}

pub fn print_license() {
    println!(include_str!("../../../LICENSE.txt"));
}

pub fn print_version(commit_id: &Option<String>, commit_date: &Option<String>) {
    match (commit_id, commit_date) {
        (Some(id), Some(date)) => {
            println!("{} {} ({} {})", crate_name!(), crate_version!(), id, date,);
        }
        _ => {
            println!("{} {}", crate_name!(), crate_version!());
        }
    }
}
