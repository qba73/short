use rand::{Rng, distr::Alphanumeric};

pub fn generate_shortcode() -> String {
    let mut rng = rand::rng();
    let short_url: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric) as char)
        .take(8)
        .collect();
    short_url
}

pub fn pprint(long_url: &str, short_url: &str) {
    println!("{: <15}: {}", "Long URL", long_url);
    println!("{: <15}: {}", "Shortened URL", short_url);
}

#[test]
fn generate_shortcode_8_characters_long() {
    let code = generate_shortcode();
    assert_eq!(8, code.len());
}
