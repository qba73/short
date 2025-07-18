use anyhow::Result;
use anyhow::bail;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use short::MAPPING_PATH;
use short::shorten;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: shorten <url>");
        bail!("invalid number of args");
    }
    let url = &args[1];

    // valid url
    if url.starts_with("http") {
        let shortcode = shorten(url)?;
        println!("{}", shortcode);
    } else {
        let file_store = File::open(MAPPING_PATH)?;
        let reader = BufReader::new(file_store);
        for line in reader.lines() {
            let mapping = line?;
            let parts: Vec<&str> = mapping.split(',').collect();
            if parts.len() != 2 {
                continue;
            }
            let short = parts[0];
            let long = parts[1];
            if short == url {
                println!("Redirecting to {}", long);
                return Ok(());
            }
        }
        println!("Short URL not found");
    }
    Ok(())
}
