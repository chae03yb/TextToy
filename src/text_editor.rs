use std::fs::File;
use crate::encoding::SupportedEncoding;

pub(crate) enum Language {
    PlainText
}

pub struct TextEditor {
    cursor_row: usize,
    cursor_col: usize,
    selected_area: (usize, usize),
    content: Vec<String>,

    language: Language,  // for syntax highlighting
    encoding: SupportedEncoding,
    file: File,
}

impl TextEditor {
    pub fn new(file: File) -> Self {
        TextEditor {
            cursor_row: 0,
            cursor_col: 0,
            selected_area: (0, 0),
            content: vec![],
            language: Language::PlainText,
            encoding: SupportedEncoding::ASCII,
            file,
        }
    }

    fn add_character(&mut self, character: char) {
        self.content[self.cursor_row].insert(self.cursor_col, character);
        self.cursor_col += 1;
    }

    fn del_character(&mut self) {
        self.content[self.cursor_row].remove(self.cursor_col);
        self.cursor_col -= 1;
    }

    fn select_area(&mut self, begin: usize, end: usize) {
        self.selected_area = (begin, end);
    }

    fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    fn set_encoding(&mut self, encoding: SupportedEncoding) {
        self.encoding = encoding;
    }

    fn open_file(&mut self, file: &File) {}
}