use clap::{arg, Arg, Command};
use std::path::PathBuf;

pub fn cli() -> Command {
    Command::new("diffi")
        .about("clea")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("jval")
                .about("Validate JSON")
                .arg(arg!(<PATH> "for JSON file to validate"))
                .arg_required_else_help(true),
        )
}
