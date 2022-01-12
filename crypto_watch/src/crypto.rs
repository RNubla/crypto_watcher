use scraper::{Html, Selector};
use serde_json::json;

pub struct Crypto {
    pub url: String,
}

trait CryptoScrapperTraits {
    fn scrape_currency_name(body_response: &String) -> Vec<String>;
    fn scrape_currency_current_price(body_response: &String) -> Vec<String>;
}
impl Crypto {
    pub fn new(url: String) -> Crypto {
        Crypto { url }
    }
    #[tokio::main]
    pub async fn scrape(&self) -> Result<(), reqwest::Error> {
        // This will fix the issue of the request being blocked by the server and returning an error code : 1020
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        let response = client.get(&self.url);
        let body_response = response.send().await?;

        let html = body_response.text().await?;

        let crypto = json!({
            "name": Crypto::scrape_currency_name(&html)[0],
            "symbol": Crypto::scrape_currency_name(&html)[1],
            "current_price": Crypto::scrape_currency_current_price(&html)[0],
        });
        println!("{}", serde_json::to_string_pretty(&crypto).unwrap());
        Ok(())
    }
}
impl CryptoScrapperTraits for Crypto {
    fn scrape_currency_name(body_response: &String) -> Vec<String> {
        let parsed_body_to_html = Html::parse_document(body_response);
        let currency_container_selector =
            Selector::parse(r#"div[class="sc-16r8icm-0 gpRPnR nameHeader"]"#).unwrap();
        let name_selector = Selector::parse(r#"h2[class="sc-1q9q90x-0 jCInrl h1"]"#).unwrap();

        let currency_content = parsed_body_to_html
            .select(&currency_container_selector)
            .next()
            .unwrap();

        let mut _currency = Vec::new();
        for currency_name in currency_content.select(&name_selector) {
            currency_name
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .for_each(|x| {
                    _currency.push(x.to_string());
                });
        }
        _currency
    }
    fn scrape_currency_current_price(body_response: &String) -> Vec<String> {
        let parsed_body_to_html = Html::parse_document(body_response);
        // let parsed_body_to_html = Html::parse_document(&body_response.text().await?);
        let price_container_selector =
            Selector::parse(r#"div[class="sc-16r8icm-0 kjciSH priceTitle"]"#).unwrap();
        let price_selector = Selector::parse("span").unwrap();

        // println!("{:?}", price_container_selector);

        let price_content = parsed_body_to_html
            .select(&price_container_selector)
            .next()
            .unwrap();
        // println!("{:?}", currency_name_content);
        let mut _price = Vec::new();
        for price in price_content.select(&price_selector) {
            price
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .for_each(|x| {
                    _price.push(x.to_string());
                });
        }
        _price
    }
}
