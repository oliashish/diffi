use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;

mod commands;

fn main() {
    let commands = commands::cli().get_matches();

    // let commands = commands::cli().get_matches();
    // println!("Hello world");
    // println!("{:?}", commands.subcommand());
    match commands.subcommand() {
        Some(("jval", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("required");
            let os_path = PathBuf::from_str(&path).unwrap();
            println!("Validating JSOn for file  with os path {:?}", os_path);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
