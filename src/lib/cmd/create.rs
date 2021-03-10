use crate::MFM_AUTHOR;
use clap::App;

pub const COMMAND_CREATE_DB_FILE: &str = "create";

pub fn create_vault_action_command<'help>() -> App<'help> {
    return App::new(COMMAND_CREATE_DB_FILE)
        .about("TODO - create vault in local storage")
        .version("0.1.1")
        .author(MFM_AUTHOR);
}
