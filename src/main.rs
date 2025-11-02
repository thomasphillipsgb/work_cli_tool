mod args;
use clap::Parser;
use work_cli_tool::{
    input_retrieval::{ConsoleInputRetrieval, InputRetrieval},
    translation_exporting::export,
    translation_importing::import,
};

use crate::args::Args;

fn main() {
    let args = Args::try_parse();
    start_interactive_mode();
    if let Err(e) = args {
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

    let input_retrieval = ConsoleInputRetrieval;

    match input_retrieval.get_user_choice(3) {
        1 => {
            println!("Exporting translations...");
            export_translations(&input_retrieval);
        }
        2 => {
            println!("Importing translations...");
            import_translations(&input_retrieval);
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

fn import_translations(input_retrieval: &ConsoleInputRetrieval) {
    let csv_path = input_retrieval
        .get_freetype_input("Enter the path to the CSV file: ")
        .expect("Failed to get CSV path");
    let resx_path = input_retrieval
        .get_freetype_input("Enter the path to the RESX file: ")
        .expect("Failed to get RESX path");

    import(&csv_path, &resx_path).unwrap();
    println!("Import completed successfully!");
}

fn export_translations(input: &dyn InputRetrieval) {
    let resx_path = input
        .get_freetype_input("Enter the path to the RESX file: ")
        .expect("Failed to get RESX path");
    let csv_path = input
        .get_freetype_input("Enter the path to the CSV file: ")
        .expect("Failed to get CSV path");

    export(&resx_path, &csv_path).unwrap();
    println!("Export completed successfully!");
}
