use std::fs;

use clap::Parser;
use forward_to_the_future::compute_price_for_movies;

use anyhow::{Context, Ok, Result};

/// Your useful tool to optimize the special Back to the Future event
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the file containing the movies' list
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Welcome to the very official CLI of this BTTF commercial event.");
    println!("Computing the final price of the cart at path {}", args.file);

    let cart_content: Vec<String> = parse_file_content(args.file).context("parse the cart content")?;

    let total_price = compute_price_for_movies(&cart_content.iter().map(|f| f.as_ref()).collect());

    println!("The total amount of the cart is {} €", total_price);

    Ok(())
}

fn parse_file_content(file: String) -> Result<Vec<String>> {
    let file_content = fs::read_to_string(file).context("read the file content")?;
    let lines : Vec<String> = file_content.lines()
        .map(String::from)
        .filter(|line| !line.is_empty())
        .collect();

    Ok(lines)
}