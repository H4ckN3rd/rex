use crossterm::event::{self, read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run() {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Event::Key(event)) => {
                    println!("{:?} \r", event);
                    match event.code {
                        KeyCode::Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                Err(err) => {
                    println!("Error reading input: {}", err);
                }
            }
        }
        disable_raw_mode().unwrap();
    }
}
