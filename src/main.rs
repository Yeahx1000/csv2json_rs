use clap::Parser;
use csv2json_rs::convert::csv_to_json;
use csv2json_rs::error::AppResult;
use csv2json_rs::types::Args;
use std::fs::File;
use std::io::{self, Write};

pub fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    let args = Args::parse();
    let file = File::open(&args.input_file)?;
    let json_array = csv_to_json(file)?;

    // if pretty print flag used, proceeds with pretty print, else, compact print
    let output = if args.pretty {
        serde_json::to_string_pretty(&json_array)?
    } else {
        serde_json::to_string(&json_array)?
    };

    match args.output_file {
        Some(path) => {
            // Writing output JSON to a specified file, if specified.
            let mut file = File::create(&path)?;
            file.write_all(output.as_bytes())?;
            println!("JSON saved to {path}");
        }
        None => {
            // Write output JSON to stdout (what would be terminal, if no output file specified)
            let mut stdout: io::StdoutLock<'_> = io::stdout().lock();
            stdout.write_all(output.as_bytes())?;
            stdout.write_all(b"\n")?;
        }
    }

    Ok(())
}
