use ratatui;
use std::env;
use std::io::{Read, stdin};

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.strip_suffix("\r\n")
            .or(input.strip_suffix("\n"))
            .unwrap_or(input.as_str());

        match input {
            "break" => break,
            _ => println!("I: {input}")
        }
    }
}

mod keyboard_input_handler;
mod text_editor;
mod test;