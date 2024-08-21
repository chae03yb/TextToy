use ratatui::crossterm::event;
use ratatui::crossterm::event::{read, Event, KeyEvent};
use std::time::Duration;

struct KeyboardInputHandler {
    poll_duration: u64
}


impl KeyboardInputHandler {
    fn event_available(&self) -> bool {
        event::poll(Duration::from_millis(self.poll_duration)).unwrap()
    }

    pub fn get_key(&self) -> Option<KeyEvent> {
        // FIXME: Caps Lock 키를 누른 후 알파벳 키를 누르면 
        //        Kind: Press는 대문자, Kind: Release는 소문자로 나타나는 문제
        // 외부 터미널에서 테스트 해야함
        if self.event_available() {
            if let Ok(Event::Key(key)) = read() {
                Option::from(key)
            } else {
                None
            }
        } else {
            None
        }
    }
}