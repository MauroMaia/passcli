use crate::MFM_AUTHOR;
use clap::App;

pub const COMMAND_ADD_PASSWORD: &str = "add-password";

pub fn add_password_action_command<'help>() -> App<'help> {
    return App::new(COMMAND_ADD_PASSWORD)
        .about("TODO - add password to the vault")
        .version("0.1.0")
        .author(MFM_AUTHOR);
}
