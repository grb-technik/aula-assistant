use aula_assistant_lib::{config::RuntimeConfigBuilder, run};
use cli::{Mode, parse_args, print_license, print_version};

#[cfg(windows)]
mod windows;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        use windows::{detach_console, was_started_from_console};

        if was_started_from_console() {
            detach_console();
        }
    }

    let mode = parse_args()?;

    match mode {
        Mode::ShowLicense => {
            print_license();
            std::process::exit(0);
        }
        Mode::ShowVersion => {
            let commit_id = option_env!("LAST_COMMIT_ID").map(str::to_string);
            let commit_date = option_env!("LAST_COMMIT_DATE").map(str::to_string);
            print_version(&commit_id, &commit_date);
            std::process::exit(0);
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
