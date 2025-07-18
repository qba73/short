use anyhow::Result;
use rand::{Rng, distr::Alphanumeric};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub fn generate_shortcode() -> String {
    let mut rng = rand::rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric) as char)
        .take(8)
        .collect()
}

pub fn pprint(long_url: &str, short_url: &str) {
    println!("{: <10}: {}", "Long URL", long_url);
    println!("{: <10}: {}", "Short URL", short_url);
}

pub fn get_shortcode(file_path: &str, url: &str) -> Result<Option<String>> {
    let file_store = File::open(file_path)?;
    let reader = BufReader::new(file_store);
    for line in reader.lines() {
        let mapping = line?;
        let parts: Vec<&str> = mapping.split(',').collect();
        if parts.len() != 2 {
            continue;
        }
        let short = parts[0];
        let long = parts[1];
        if long == url {
            return Ok(Some(short.to_string()));
        }
    }
    return Ok(None);
}

pub const MAPPING_PATH: &str = "src/mapping.txt";

pub fn shorten(url: &str) -> Result<String> {
    // Verify for duplicates
    let exist = get_shortcode(MAPPING_PATH, url)?;

    if let Some(shortcode) = exist {
        return Ok(shortcode);
    }
    let short_url = generate_shortcode();

    // storage
    let mut file_store = OpenOptions::new()
        .write(true)
        .append(true)
        .open(MAPPING_PATH)?;

    let mapping = format!("{},{}\n", short_url, url);
    file_store.write_all(mapping.as_bytes())?;
    Ok(short_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shorten_returns_shortcode_for_existing_url() {
        let short_url = shorten("http://wp.pl").unwrap();
        assert_eq!("SyrRUXBT", short_url);
    }

    #[test]
    fn generate_shortcode_8_characters_long() {
        let code = generate_shortcode();
        assert_eq!(8, code.len());
    }
}
