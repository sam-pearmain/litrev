use clap::{CommandFactory, Parser, Subcommand};
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::PathBuf;

mod bibtex;

const DB_FILE: &str = "literature/papers.json";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize the project by creating a 'literature' directory
    Init,
    /// Add a new paper to the library from a BibTex file
    Add,
    /// List all papers in the library
    List,
    /// Open the PDF for a specific paper
    Open,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Init => todo!(), 
            Commands::Add  => todo!(),
            Commands::List => todo!(),
            Commands::Open => todo!(),
        }
    } else {
        Cli::command().print_help()?;
    }

    Ok(())
}