use crate::MFM_AUTHOR;
use clap::App;

pub const COMMAND_REMOVE_ENTRY: &str = "remove";

pub fn remove_password_action_command<'help>() -> App<'help> {
    return App::new(COMMAND_REMOVE_ENTRY)
        .about("TODO - remove.rs password from the vault")
        .version("0.1.0")
        .author(MFM_AUTHOR);
}
