use crate::MFM_AUTHOR;
use clap::{App, Arg};

pub const COMMAND_GENERATE_PASSWORD: &str = "generate";

const POSSIBLE_CHARSETS: [&str; 4] = [&"full", &"numeric", &"alphanumeric", &"chars-only"];

pub fn generate_password_action_command<'help>() -> App<'help> {
    return App::new(COMMAND_GENERATE_PASSWORD)
        .about("This subcommand generates a random password. Different charsets can be used.")
        .version("0.1.1")
        .author(MFM_AUTHOR)
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .about("Password length")
                .default_value("8")
                .number_of_values(1), //.validator(lib::validators::isOneDigit)
        )
        .arg(
            Arg::new("charset")
                .short('c')
                .long("charset")
                .about("charset to use")
                .default_value("full")
                .possible_values(&POSSIBLE_CHARSETS)
                .number_of_values(1), //.validator(lib::validators::isOneDigit)
        );
}
