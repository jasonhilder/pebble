mod builder;

use clap::{crate_version, Arg, Command};
use std::path::PathBuf;
use std::{env, fs, process};

pub struct Test {

}

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
                .help("Directory path of project"),
        )
        .subcommand(Command::new("build").about("build the site"))
}

pub fn cleanup_and_exit() {
    fs::remove_dir_all("build").unwrap();
    process::exit(0)
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
        Some(("build", _)) => builder::build(&project_path),
        _ => unreachable!(),
    }
}
