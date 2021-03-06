use clap::{crate_version,Arg, App, AppSettings};

mod lib;

const MFM_AUTHOR: &str = "Mauro Maia <dev@maurofilipemaia.dev>";

fn add_password_action_command<'help>() -> App<'help> {
    return App::new(lib::add_password::COMMAND_ID)
        .about("TODO - add password to the vault")
        .version("0.1.0")
        .author(MFM_AUTHOR);
    //.arg();
}

fn remove_password_action_command<'help>() -> App<'help> {
    return App::new(lib::remove_password::COMMAND_ID)
        .about("TODO - remove password from the vault")
        .version("0.1.0")
        .author(MFM_AUTHOR);
    //.arg();
}


fn generate_password_action_command<'help>() -> App<'help> {
    return App::new(lib::generate_password::COMMAND_ID)
        .about("This subcommand generates a random password. Different charsets can be used.")
        .version("0.1.1")
        .author(MFM_AUTHOR)
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .about("Password length")
                .default_value("8")
                .number_of_values(1)
            //.validator(lib::validators::isOneDigit)
        ).arg(
        Arg::new("charset")
            .short('c')
            .long("charset")
            .about("charset to use")
            .default_value("full")
            .possible_values(&lib::generate_password::POSSIBLE_CHARSETS)
            .number_of_values(1)
        //.validator(lib::validators::isOneDigit)
    );
}

fn create_vault_action_command<'help>() -> App<'help> {
    return App::new(lib::create_file::COMMAND_ID)
        .about("TODO - create vault in local storage")
        .version("0.1.0")
        .author(MFM_AUTHOR);
    //.arg();
}


fn main() {
    let app = App::new("Pass(word) manager program")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author(MFM_AUTHOR)
        .about("Does awesome things. Or not!!!")
        .arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .about("Sets the level of verbosity"))
        .subcommand(generate_password_action_command())
        .subcommand(create_vault_action_command())
        .subcommand(add_password_action_command())
        .subcommand(remove_password_action_command());

    let matches = app
        .get_matches();

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
    if let Some(ref matches) = matches.subcommand_matches(lib::generate_password::COMMAND_ID) {
        let size = matches.value_of("size").unwrap_or("8");
        if !size.parse::<usize>().err().is_none() {
            // TODO -  move this to an validator
            println!("Can't parse size value: {:?}", size);
        }

        let password: String = lib::generate_password::generate_random_password(
            size.parse::<usize>().unwrap(),
            matches.value_of("charset").unwrap(),
        );
        println!("Password: {:?}", password);
    } else if let Some(ref matches) = matches.subcommand_matches(lib::create_file::COMMAND_ID) {
        lib::create_file::create_file()
    } else {
        println!(
            "subcommand {:?} is unavailable at the moment. It will be implemented as soon as possible. Sorry for the wait.", matches.subcommand().unwrap());
    }
}
