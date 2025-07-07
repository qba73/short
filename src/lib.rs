use rand::{Rng, distr::Alphanumeric};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn generate_shortcode() -> String {
    let mut rng = rand::rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric) as char)
        .take(8)
        .collect()
}

pub fn pprint(long_url: &str, short_url: &str) {
    println!("{: <15}: {}", "Long URL", long_url);
    println!("{: <15}: {}", "Shortened URL", short_url);
}

pub fn check_duplicate(file_path: &str, url: &str) -> bool {
    let file_store = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("error opening file store");
            return false;
        }
    };
    let reader = BufReader::new(file_store);
    for line in reader.lines() {
        let mapping = match line {
            Ok(line) => line,
            Err(_) => {
                println!("error reading file store");
                continue;
            }
        };
        let parts: Vec<&str> = mapping.split(',').collect();
        if parts.len() != 2 {
            continue;
        }
        let short = parts[0];
        let long = parts[1];
        if long == url {
            println!("URL {} already shortened", url);

            pprint(long, short);

            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_shortcode_8_characters_long() {
        let code = generate_shortcode();
        assert_eq!(8, code.len());
    }

    #[test]
    fn check_duplicates_detect_duplicate_entry() {
        let is_duplicated = check_duplicate("test/mapping.txt", "http://wp.pl");
        assert_eq!(true, is_duplicated);
    }

    #[test]
    fn check_duplicates_when_no_duplicates() {
        let is_duplicated = check_duplicate("test/mapping.txt", "http://hello.pl");
        assert_eq!(false, is_duplicated);
    }
}
