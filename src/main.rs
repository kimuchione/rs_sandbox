use crossterm::event::{read, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers};
use reqwest::blocking::Client;

pub const API_KEY: &str = include_str!("../../api.txt");
struct FinanceClient {
    url: String,
    client: Client,
}

fn main() -> crossterm::Result<()> {
    let individual_client = FinanceClient {
        url: "https://finnhub.io/api/v1".to_string(),
        client: Client::default(),
    };
    let FinanceClient { url, client } = individual_client;

    let text = client.get(url).send().unwrap().text().unwrap();
    println!("Text : {}", text);

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
