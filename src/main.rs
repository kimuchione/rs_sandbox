use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub const API_KEY: &str = include_str!("../../api.txt");

struct FinanceClient {
    url: String,
    client: Client,
    search_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompanyInfo {
    country: String,
    currency: String,
    exchange: String,
    #[serde(rename = "finnhubIndustry")]
    finnhub_industry: String,
    ipo: String,
    #[serde(rename = "marketCapitalization")]
    market_capitalization: f64,
    name: String,
    ticker: String,
    weburl: String,
}

impl std::fmt::Display for CompanyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Country : {}\nCurrency : {}\nExchange : {}\nIndustry : {}\nIpo : {}\nMarketCapitalization : {}\nName : {}\nTicker : {}\nUrl : {}",
            self.country, self.currency, self.exchange, self.finnhub_industry, self.ipo,self.market_capitalization,self.name,self.ticker,self.weburl
        )
    }
}

impl FinanceClient {
    fn get_profile(&self) {
        let text = &self
            .client
            .get(format!(
                "{}/stock/profile2?symbol={}",
                &self.url, &self.search_string
            ))
            .header("X-Finnhub-Token", API_KEY)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let company_info: CompanyInfo = serde_json::from_str(&text).unwrap();
        println!("{company_info}");
    }
}

fn main() -> crossterm::Result<()> {
    let mut individual_client = FinanceClient {
        url: "https://finnhub.io/api/v1/".to_string(),
        client: Client::default(),
        search_string: String::new(),
    };

    loop {
        match read()? {
            Event::Key(event) => {
                let KeyEvent { code, modifiers } = event;
                match (code, modifiers) {
                    (KeyCode::Char(c), modifier) => {
                        individual_client.search_string.push(c);
                        println!("{}", individual_client.search_string);
                    }
                    (KeyCode::Esc, _) => {
                        individual_client.search_string.clear();
                        println!("{}", individual_client.search_string);
                    }
                    (KeyCode::Backspace, _) => {
                        individual_client.search_string.pop();
                        println!("{}", individual_client.search_string);
                    }
                    (KeyCode::Enter, _) => individual_client.get_profile(),
                    (KeyCode::Up, _) => println!("Pressed Up!"),
                    (KeyCode::Down, _) => println!("Pressed Down!"),
                    (KeyCode::Left, _) => println!("Pressed Left!"),
                    (KeyCode::Right, _) => println!("Pressed Right!"),
                    _ => {}
                }
            }
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
}
