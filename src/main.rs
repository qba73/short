use rand::{Rng, distr::Alphanumeric};
use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

fn main() {
    let mapping_path = "src/mapping.txt";
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: shorten <url>");
        return;
    }

    let url = &args[1];

    // valid url
    if url.starts_with("http") {
        // Verify for duplicates
        let exist = check_duplicate(mapping_path, url);
        if exist {
            return;
        }

        let mut rng = rand::rng();
        let short_url: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) as char)
            .take(8)
            .collect();

        pprint(url, &short_url);

        // storage
        let mut file_store = match OpenOptions::new()
            .write(true)
            .append(true)
            .open(mapping_path)
        {
            Ok(file) => file,
            Err(_) => {
                println!("error opening file store for URLs");
                return;
            }
        };

        let mapping = format!("{},{}\n", short_url, url);
        if let Err(_) = file_store.write_all(mapping.as_bytes()) {
            println!("error writing to file store");
            return;
        }
    } else {
        let file_store = match File::open(mapping_path) {
            Ok(file) => file,
            Err(_) => {
                println!("error opening file store");
                return;
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
            if short == url {
                println!("Redirecting to {}", long);
                return;
            }
        }
        println!("Short URL not found");
    }
}

fn check_duplicate(file_path: &str, url: &String) -> bool {
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
        let long = parts[1];
        if long == url {
            println!("URL {} already shortened", url);
            return true;
        }
    }
    return false;
}

fn pprint(long_url: &String, short_url: &String) {
    println!("{: <15}: {}", "Long URL", long_url);
    println!("{: <15}: {}", "Shortened URL", short_url);
}
