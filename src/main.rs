use clap::{crate_version, App, AppSettings, Arg};
use std::process;

mod lib;

use crate::lib::cmd::create::{create_vault_action_command, COMMAND_CREATE_DB_FILE};
use crate::lib::cmd::entry::add_password_action_command;
use crate::lib::cmd::generate::{generate_password_action_command, COMMAND_GENERATE_PASSWORD};
use crate::lib::cmd::remove::remove_password_action_command;

const MFM_AUTHOR: &str = "Mauro Maia <dev@maurofilipemaia.dev>";

fn main() {
    let app = App::new("Pass(word) manager cli interface")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author(MFM_AUTHOR)
        .about("\nDoes awesome things. Or not!!!\n")
        .arg(
            Arg::new("v")
                .short('v')
                .multiple(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(generate_password_action_command())
        .subcommand(create_vault_action_command())
        .subcommand(add_password_action_command())
        .subcommand(remove_password_action_command());

    let matches = app.get_matches();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("v") {
        0 => print!(""), // Verbose mode is off
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches(COMMAND_GENERATE_PASSWORD) {
        let size = matches.value_of("size").unwrap_or("8");
        if !size.parse::<usize>().err().is_none() {
            // TODO -  move this to an validator
            println!("Can't parse size value: {:?}", size);
        }

        let password: String = lib::core::generate_password::generate_random_password(
            size.parse::<usize>().unwrap(),
            matches.value_of("charset").unwrap(),
        );
        println!("Password: {:?}", password);
    } else if let Some(ref matches) = matches.subcommand_matches(COMMAND_CREATE_DB_FILE) {
        lib::core::create_file::create_file_with_password(
            "/tmp/test_db_with_password.kdbx",
            "password",
        )
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });
    } else {
        println!(
            "subcommand {:?} is unavailable at the moment. It will be implemented as soon as possible. Sorry for the wait.", matches.subcommand().unwrap());
    }
}
