use anyhow::Result;
use clap::Parser;
use reqwest::header;
use std::io::Cursor;

extern crate dotenv;
extern crate reqwest;

#[macro_use]
extern crate dotenv_codegen;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser, Debug)]
enum Subcommand {
    /// Load the given days input
    Load { day: u8 },
    /// Show the usage of aoc
    Usage,
}

fn usage() {
    println!(
        "These are my solutions for aoc 2023.
Each day is it's own binary so you can run it using:
    cargo run --bin day[1-25]

Each day also has tests using the examples.
These can be run using:
    cargo test --bins

Or a single day with:
    cargo test --bin day[1-25]"
    );
}

async fn fetch_url(url: String, file_name: String, cookie: String) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(header::COOKIE, format!("session={cookie}"))
        .send()
        .await?;

    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(res.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cookie = dotenv!("AOC_COOKIE");

    match args.subcommand {
        Subcommand::Load { day } => {
            if (fetch_url(
                format!("https://adventofcode.com/2023/day/{day}/input"),
                format!("data/{day}.input"),
                cookie.into(),
            )
            .await)
                .is_ok()
            {
                println!("Input file saved as '{day}.input'.")
            } else {
                eprintln! {"Error downloading input file for day{day}."}
            }
        }
        Subcommand::Usage => usage(),
    }
}
