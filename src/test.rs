#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_push() {
        let mut content: Vec<String> = vec![ String::from("llo, world!") ];
        let mut cursor_col: usize = 0;
        let cursor_row: usize = 0;

        fn ac(c: &mut Vec<String>, row: usize, col: &mut usize, ch: char) {
            c[row].insert(*col, ch);
            *col += 1;
        }

        ac(&mut content, cursor_row, &mut cursor_col, 'H');
        ac(&mut content, cursor_row, &mut cursor_col, 'e');

        assert_eq!(2, cursor_col);
        assert_eq!(String::from("Hello, world!"), content[0]);
    }

    #[test]
    fn string_pop() {
        let mut content: Vec<String> = vec![ String::from("Hiello, world!") ];
        let mut cursor_col: usize = 1;
        let cursor_row: usize = 0;

        fn dc(c: &mut Vec<String>, row: usize, col: &mut usize) {
            c[row].remove(*col);
            *col -= 1;
        }

        dc(&mut content, cursor_row, &mut cursor_col);

        assert_eq!(0, cursor_col);
        assert_eq!(String::from("Hello, world!"), content[0]);
    }
}