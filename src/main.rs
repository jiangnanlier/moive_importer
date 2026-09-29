mod movie_converter;

use movie_converter::read_txt_file_to_json;
use rfd::FileDialog;
use std::{error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let Some(file_path) = FileDialog::new()
        .add_filter("Text", &["txt"])
        .set_title("Select a text file")
        .set_directory(".")
        .pick_file()
    else {
        eprintln!("No file selected.");
        process::exit(1);
    };

    println!("Selected file: {}", file_path.display());
    let saved_path = read_txt_file_to_json(&file_path)?;
    println!("Saved path: {}", saved_path.display());
    Ok(())
}
