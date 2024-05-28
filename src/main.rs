mod config;
mod replace;

use std::{env, path::PathBuf};

use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use config::{evaluate_config, read_config};

fn main() {
    let matches = Command::new("binjector")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .about("simple base 16 injector")
        .subcommand(
            Command::new("build")
                .short_flag('B')
                .long_flag("build")
                .about("evaluate config")
                .arg(
                    Arg::new("config")
                        .long("config")
                        .help("custom config path")
                        .action(ArgAction::Set),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("build", build_matches)) => {
            let mut config_path = env::current_dir()
                .expect("no working directory")
                .join("config.toml");

            if build_matches.contains_id("config") {
                config_path = PathBuf::from(build_matches.get_one::<String>("config").unwrap())
            }

            match read_config(config_path.to_str().unwrap().to_string()) {
                Ok(config) => match evaluate_config(config) {
                    Err(e) => eprintln!("{}", e.to_string().as_str().red()),
                    _ => println!("replaced"),
                },
                Err(e) => println!(
                    "{}",
                    format!(
                        "error while reading config at {},\n {:?}",
                        config_path.to_str().unwrap(),
                        e
                    )
                    .red()
                ),
            }
        }
        _ => unreachable!(),
    }
}
