mod movie_converter;

use clap::{Parser, Subcommand};
use movie_converter::read_txt_file_to_json;
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(version, about = "Convert movie lists to JSON")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(value_name = "INPUT.txt")]
        input: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Convert { input } => {
            let saved_path = read_txt_file_to_json(&input)?;
            println!("Saved path: {}", saved_path.display());
        }
    }
    Ok(())
}
