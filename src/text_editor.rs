use std::fs::File;

enum Language {
    PlainText
}

enum Encoding {
    UTF8,
    ASCII,
}

struct TextEditor {
    cursor_row: usize,
    cursor_col: usize,
    selected_area: Vec<usize>,
    content: Vec<String>,

    language: Language,  // for syntax highlighting
    encoding: Encoding,
    file: File,
}

impl TextEditor {
    fn new(file: File) -> Self {
        TextEditor {
            cursor_row: 0,
            cursor_col: 0,
            selected_area: vec![],
            content: vec![],
            language: Language::PlainText,
            encoding: Encoding::ASCII,
            file
        }
    }

    fn add_character(&mut self, character: char) {

    }

    fn del_character(&mut self) {

    }

    fn select_area(&mut self, begin: usize, end: usize) {

    }

    fn set_language(&mut self, language: Language) {

    }

    fn set_encoding(&mut self, encoding: Encoding) {

    }

    fn open_file(&mut self, file: &File) {

    }
}