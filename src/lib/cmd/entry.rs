use crate::MFM_AUTHOR;
use clap::{App, Arg};

pub const COMMAND_ADD_ENTRY: &str = "add-entry";
pub const COMMAND_LIST_ENTRIES: &str = "list-entries";

pub fn add_entry_action_command<'help>() -> App<'help> {
    return App::new(COMMAND_ADD_ENTRY)
        .about("Create a new entry in the database")
        .version("0.1.0")
        .author(MFM_AUTHOR)
        .arg(
            Arg::new("vault_path")
                .short('v')
                .long("vault_path")
                .about("Path to the vault database")
                .required(true)
                .number_of_values(1),
        )
        .arg(
            Arg::new("title")
                .short('t')
                .long("title")
                .about("Entry title")
                .required(true)
                .number_of_values(1),
        )
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .about("Entry username")
                .number_of_values(1),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .about("Entry password")
                .number_of_values(1),
        )
        .arg(
            Arg::new("url")
                .short('U')
                .long("url")
                .about("Entry url")
                .number_of_values(1),
        );
}
