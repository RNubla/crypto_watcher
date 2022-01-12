use clap::{App, Arg};

use crypto_watch::crypto::Crypto;
// mod crypto;
// use crypto::Crypto;
fn main() {
    let matches = App::new("Crypto Watcher")
        .version("0.1.0")
        .author("Robert Nubla")
        .about("Watches for crypto currency prices")
        .arg(
            Arg::new("currency")
                .short('c')
                .long("currency")
                .takes_value(true)
                .help("Show current price for a currency that you specify"),
        )
        .get_matches();

    if matches.is_present("currency") {
        let url = format!(
            "https://coinmarketcap.com/currencies/{}/",
            matches.value_of("currency").unwrap()
        );
        Crypto::new(url).scrape().unwrap();
    }
}

// TODO: Instead of using the full name of the crypto, use the symbol
