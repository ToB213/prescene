mod error;
mod input;
mod model;
mod output;
mod renderer;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::error::AppError;

// Command-line options accepted by the application.
#[derive(Debug, Parser)]
#[command(name = "prescene", about = "A simple presentation generator")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

// Each subcommand represents one application workflow.
#[derive(Debug, Subcommand)]
enum Command {
    Build {
        input: PathBuf,

        #[arg(short = 'c', long = "css")]
        css_paths: Vec<PathBuf>,

        #[arg(short, long, default_value = "output/index.html")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    // Keep error handling at the process boundary so the inner functions can
    // use Result and the caller receives a non-zero exit status on failure.
    if let Err(error) = run(cli) {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }
}

// Dispatch the selected CLI subcommand to its implementation.
fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Command::Build {
            input,
            output,
            css_paths,
        } => build(input, output, css_paths),
    }
}

// Build an HTML presentation from a YAML input file.
fn build(input: PathBuf, output: PathBuf, cli_css_paths: Vec<PathBuf>) -> Result<(), AppError> {
    let document = input::load_input(&input)?;

    let mut css_paths = document.css_paths;
    css_paths.extend(cli_css_paths);

    let custom_css = input::load_css(&css_paths)?;
    let html = renderer::render_html(&document.presentation, &custom_css);

    output::write_html(&output, &html)?;

    Ok(())
}
