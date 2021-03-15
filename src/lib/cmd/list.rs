use crate::MFM_AUTHOR;
use clap::{App, Arg};

pub const COMMAND_LIST_ENTRIES: &str = "list-entries";
pub const COMMAND_LIST_GROUPS: &str = "list-groups";

pub fn list_entries_action_command<'help>() -> App<'help> {
    return App::new(COMMAND_LIST_ENTRIES)
        .about("This subcommand list all saved passwords")
        .version("0.1.0")
        .author(MFM_AUTHOR)
        .arg(
            Arg::new("vault_path")
                .short('v')
                .long("vault_path")
                .about("Path to the vault database")
                .required(true)
                .number_of_values(1),
        );
    //TODO -  add FORMAT JSON/TABLE???
}
