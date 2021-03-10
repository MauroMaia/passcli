use crate::MFM_AUTHOR;
use clap::{App, Arg};

pub const COMMAND_CREATE_DB_FILE: &str = "create";

pub fn create_vault_action_command<'help>() -> App<'help> {
    return App::new(COMMAND_CREATE_DB_FILE)
        .about("Create the vault  database (currently only possible in local storage)")
        .version("0.1.2")
        .author(MFM_AUTHOR)
        .arg(
            Arg::new("vault_path")
                .short('v')
                .long("vault_path")
                .about("Path to the vault database")
                .required(true)
                .number_of_values(1),
        );
}
