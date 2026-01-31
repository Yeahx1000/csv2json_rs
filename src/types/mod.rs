use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "csv2json-rs",
    version,
    about = "CLI tool for converting CSV files to JSON arrays"
)]
pub struct Args {
    pub input_file: String,
    #[arg(long)]
    pub pretty: bool,
    #[arg(long)]
    pub output_file: Option<String>,
}
