use ratatui;

use std::env;
use std::fs::File;
use std::io::{Read};
use std::path::Path;
use crate::app::d;

fn main() {
    // loop {
    //     let mut input = String::new();
    //     stdin().read_line(&mut input).unwrap();
    //     let input = input.strip_suffix("\r\n")
    //         .or(input.strip_suffix("\n"))
    //         .unwrap_or(input.as_str());
    //
    //     match input {
    //         "break" => break,
    //         _ => println!("I: {input}")
    //     }
    // }

    // let args: Vec<String> = env::args().collect();
    //
    // let fs = File::open(Path::new(&args[1]));
    // let fs = match fs {
    //     Ok(file) => file,
    //     Err(error) => panic!("Cannot open file: {}", error)
    // };
    //
    // let text_editor = text_editor::TextEditor::new(fs);
}

mod keyboard_input_handler;
pub mod text_editor;
mod test;
mod app;
mod ui;