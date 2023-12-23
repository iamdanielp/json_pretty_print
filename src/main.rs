use std::collections::HashMap;
use std::fs;
use clap::Parser;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    out: String,
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    println!("Reading {}...", args.file);

    let data = fs::read_to_string(args.file).expect("Unable to open input file.");
    let json: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
    let pretty = serde_json::to_string_pretty(&json).unwrap();

    fs::write(&args.out, pretty).expect("Unable to write to output file.");

    println!("Successfully wrote file {}", &args.out);

    return Ok(());
}