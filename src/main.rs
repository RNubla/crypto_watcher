use clap::{App, Arg};
use reqwest::Response;
use scraper::{Html, Selector};

fn main() {
    let matches = App::new("Crypto Watcher")
        .version("0.1.0")
        .author("Robert Nubla")
        .about("Watches for crypto currency prices")
        .arg(
            Arg::new("currency")
                .short('c')
                .long("currentcy")
                .takes_value(true)
                .help("Show current price for a currency that you specify"),
        )
        .get_matches();

    if matches.is_present("currency") {
        let url = format!(
            "https://coinmarketcap.com/currencies/{}/",
            matches.value_of("currency").unwrap()
        );
        scrape(&url).unwrap();
    }
}

// async fn scrape_currency_name(body_response: Response) -> Result<(), reqwest::Error> {
fn scrape_currency_name(body_response: &String) -> String {
    // let parsed_body_to_html = Html::parse_document(&body_response.text().await?);
    let parsed_body_to_html = Html::parse_document(body_response);
    let currency_container_selector =
        Selector::parse(r#"div[class="sc-16r8icm-0 gpRPnR nameHeader"]"#).unwrap();
    let currency_selector = Selector::parse("h2").unwrap();

    let currency_content = parsed_body_to_html
        .select(&currency_container_selector)
        .next()
        .unwrap();
    // for curren
    // println!("{:?}", currency_content);
    // let mut currency_name_str = String::new();
    let mut _currency = Vec::new();
    for currency_name in currency_content.select(&currency_selector) {
        // currency_name_str.push_str(currency_name.text().collect::<Vec<_>>().join(""));
        _currency.push(currency_name.text().collect::<Vec<_>>().join(" "));
        // _currency.push(currency_name.text());
    }
    _currency.join(" ")
    // println!("{:?}", _currency);
    // let t = _currency[0];
    // for currency_name in currency_content.select(&currency_selector) {
    //     println!(
    //         "{}",
    //         format!("{}", currency_name.text().collect::<Vec<_>>().join(" "))
    //     );
    // }
    // Ok(())
}

// async fn scrape_currency_current_price(body_response: Response) -> Result<(), reqwest::Error> {
// async fn scrape_currency_current_price(body_response: String) -> Result<(), reqwest::Error> {
fn scrape_currency_current_price(body_response: &String) -> String {
    let parsed_body_to_html = Html::parse_document(body_response);
    // let parsed_body_to_html = Html::parse_document(&body_response.text().await?);
    let price_container_selector = Selector::parse(r#"div[class="priceValue"]"#).unwrap();
    let price_selector = Selector::parse("span").unwrap();

    println!("{:?}", price_container_selector);

    let price_content = parsed_body_to_html
        .select(&price_container_selector)
        .next()
        .unwrap();
    // println!("{:?}", currency_name_content);
    let mut _price = Vec::new();
    for price in price_content.select(&price_selector) {
        _price.push(price.text().collect::<Vec<_>>().join(" "));
        // String::from("{}", price.text().collect::<Vec<_>>().join(" "));
        // println!(
        //     "{}",
        //     format!("{}", price.text().collect::<Vec<_>>().join(" "))
        // );
    }
    _price.join(" ")
}

#[tokio::main]
async fn scrape(url: &str) -> Result<(), reqwest::Error> {
    // This fixes the issue of error code : 1020 due to website detecting bots for scrapping
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let response = client.get(url);
    let body_response = response.send().await?;

    let html = body_response.text().await?;
    // println!("{}", html);
    println!("{}", scrape_currency_name(&html));
    println!("{}", scrape_currency_current_price(&html));

    // scrape_currency_current_price(body_response).await?;

    // println!("{:?}", body_response.text().await?);
    // scrape_currency_current_price(html).await?;
    // scrape_currency_name(body_response).await?;
    // scrape_currency_name(&html);
    // println!("{:?}", scrape_currency_name(&html));
    // scrape_currency_current_price(body_response).await?;

    /* let parsed_body_to_html = Html::parse_document(&body_response.text().await?);
    let currency_selector =
        Selector::parse(r#"div[class="sc-16r8icm-0 gpRPnR nameHeader"]"#).unwrap();
    let currency_name_selector = Selector::parse("h2").unwrap();

    let currency_name_content = parsed_body_to_html
        .select(&currency_selector)
        .next()
        .unwrap();
    // println!("{:?}", currency_name_content);
    for currency_name in currency_name_content.select(&currency_name_selector) {
        println!(
            "{}",
            format!("{}", currency_name.text().collect::<Vec<_>>().join(" "))
        );
    } */

    Ok(())
}
