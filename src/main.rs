use clap::{crate_version, App, AppSettings, Arg, ArgMatches};
use std::process;

mod lib;

use crate::lib::cmd::create::{create_vault_action_command, COMMAND_CREATE_DB_FILE};
use crate::lib::cmd::entry::{add_entry_action_command, COMMAND_ADD_ENTRY};
use crate::lib::cmd::generate::{generate_password_action_command, COMMAND_GENERATE_PASSWORD};
use crate::lib::cmd::list::{list_entries_action_command, COMMAND_LIST_ENTRIES};
use crate::lib::cmd::remove::remove_password_action_command;
use crate::lib::core::entry::Entry;

const MFM_AUTHOR: &str = "Mauro Maia <dev@maurofilipemaia.dev>";

fn handle_command_generate_password(matches: &ArgMatches) {
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
}

fn handle_command_create_db_file(matches: &ArgMatches) {
    lib::core::vault::create_file_with_password(
        matches.value_of("vault_path").unwrap(),
        // FIXME - remove _or from unwrap and set password as a required argument
        matches.value_of("password").unwrap_or("password"),
    )
    .unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}

fn handle_command_add_entry(matches: &ArgMatches) {
    let entry = &Entry {
        title: matches.value_of("title").unwrap(),
        username: matches.value_of("username").unwrap_or(""),
        password: matches.value_of("password").unwrap_or(""),
        url: matches.value_of("url").unwrap_or(""),
        group: "",
    };

    lib::core::entry::add_entry(
        matches.value_of("vault_path").unwrap(),
        // FIXME - remove hardcode password value
        "password",
        entry,
    )
    .unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}

fn handle_command_list_entries(matches: &ArgMatches) {
    lib::core::entry::list_all_entries(matches.value_of("vault_path").unwrap(), "password")
}

fn missing_command(matches: &ArgMatches) {
    println!(
        "subcommand {:?} is unavailable at the moment. It will be implemented as soon as possible. Sorry for the wait.", matches.subcommand().unwrap())
}

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
        .subcommand(add_entry_action_command())
        .subcommand(list_entries_action_command())
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
    match matches.subcommand_name().unwrap_or("") {
        COMMAND_GENERATE_PASSWORD => handle_command_generate_password(
            matches
                .subcommand_matches(COMMAND_GENERATE_PASSWORD)
                .unwrap(),
        ),
        COMMAND_CREATE_DB_FILE => handle_command_create_db_file(
            matches.subcommand_matches(COMMAND_CREATE_DB_FILE).unwrap(),
        ),
        COMMAND_ADD_ENTRY => {
            handle_command_add_entry(matches.subcommand_matches(COMMAND_CREATE_DB_FILE).unwrap())
        }
        COMMAND_LIST_ENTRIES => {
            handle_command_list_entries(matches.subcommand_matches(COMMAND_LIST_ENTRIES).unwrap())
        }
        _ => missing_command(&matches),
    }
}
