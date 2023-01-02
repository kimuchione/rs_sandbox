use crossterm::event::{read, EnableMouseCapture, Event, KeyCode, KeyEvent};

fn main() -> crossterm::Result<()> {
    loop {
        match read()? {
            Event::Key(event) => {
                let KeyEvent { code, modifiers } = event;
                match (code, modifiers) {
                    (KeyCode::Char(c), _) if c == ' ' => println!("empty"),
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
