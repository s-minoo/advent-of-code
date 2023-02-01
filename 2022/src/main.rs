use std::error::Error;
use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use year2022::days;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let day = cli.day;
    let input_file = File::open(cli.input)?;
    let buf_read = BufReader::new(input_file);

    days::solve_day(day, buf_read);

    Ok(())
}
