mod builder;

use std::env;
use std::path::PathBuf;
use clap::{Command, Arg, crate_version};

pub fn cli() -> Command<'static> {
    Command::new("pebble")
        .about("A small static site generator")
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .takes_value(true)
                .default_value(".")
                .help("Directory path of project")
        )
        .subcommand(
            Command::new("build")
                .about("build the site")
        )
}

fn main() {
    let matches = cli().get_matches();

    let project_path = match matches.value_of("path").unwrap() {
        "." => env::current_dir().unwrap(),
        path => PathBuf::from(path)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Cannot find directory at path: {}", path)),
    };

    match matches.subcommand() {
        Some(("build", _)) => {
            builder::build(&project_path)
        }
        _ => unreachable!(),
    }
}
