use rand::{Rng, distr::Alphanumeric};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_shortcode_8_characters_long() {
        let code = generate_shortcode();
        assert_eq!(8, code.len());
    }
}
