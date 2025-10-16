mod args;
use clap::Parser;

use crate::args::Args;

fn main() {
    let args = Args::try_parse();
    if let Err(e) = args {
        start_interactive_mode();
    } else {
        let _args = args.unwrap();
        // Handle command-line arguments here
    }
}

fn start_interactive_mode() {
    println!("Welcome to the greatest CLI tool!");
    println!("1. Export Translations");
    println!("2. Import Translations");
    println!("3. App Versioning");

    match get_user_choice() {
        1 => {
            println!("Exporting translations...");
            export_translations();
        }
        2 => {
            println!("Importing translations...");
            todo!();
        }
        3 => {
            println!("Showing app version...");
            todo!();
        }
        _ => {
            println!("Invalid choice, please enter a number between 1 and 3.")
        }
    }
}

fn export_translations() {

    todo!()
}

fn get_user_choice() -> i32 {
    use std::io;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<i32>() {
        Ok(num) => num,
        _ => {
            println!("Invalid choice, please enter a number between 1 and 3.");
            get_user_choice()
        }
    }
}
