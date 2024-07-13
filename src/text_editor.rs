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
    selected_area: (usize, usize),
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
            selected_area: (0, 0),
            content: vec![],
            language: Language::PlainText,
            encoding: Encoding::ASCII,
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

    fn set_encoding(&mut self, encoding: Encoding) {
        self.encoding = encoding;
    }

    fn open_file(&mut self, file: &File) {}
}