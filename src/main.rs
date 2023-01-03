use crossterm::event::{read, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers};
use reqwest::blocking::Client;

pub const API_KEY: &str = include_str!("../../api.txt");
#[derive(Debug)]
struct FinanceClient {
    url: String,
    client: Client,
}

impl FinanceClient {
    fn get_profile(&self, symbol: &str) {
        let text = &self
            .client
            .get(format!("{}/stock/profile2?symbol={symbol}", &self.url))
            .header("X-Finnhub-Token", API_KEY)
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("Text : {text}");
    }
}

fn main() -> crossterm::Result<()> {
    let individual_client = FinanceClient {
        url: "https://finnhub.io/api/v1/".to_string(),
        client: Client::default(),
    };
    individual_client.get_profile("TSLA");

    loop {
        match read()? {
            Event::Key(event) => {
                let KeyEvent { code, modifiers } = event;
                match (code, modifiers) {
                    (KeyCode::Char(c), _) if c == ' ' => println!("Pressd Spacebar!"),
                    (KeyCode::Char(c), modifier)
                        if c == 's' && modifier == KeyModifiers::CONTROL =>
                    {
                        println!("saved")
                    }
                    (KeyCode::Char(c), _) => println!("Got a char : {}", c),
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
