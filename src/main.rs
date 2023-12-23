use std::collections::HashMap;
use std::fs;
use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct U;

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

    let data = fs::read_to_string(args.file).expect("Unable to open input file.");
    let json: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
    let pretty = serde_json::to_string_pretty(&json).unwrap();

    println!("{:?}", pretty);

    fs::write(args.out, pretty).expect("Unable to write to output file.");

    return Ok(());
}