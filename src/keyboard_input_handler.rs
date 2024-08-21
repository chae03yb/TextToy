use ratatui::crossterm::event;
use ratatui::crossterm::event::{read, Event, KeyEvent};
use std::time::Duration;

struct KeyboardInputHandler {
    poll_rate: u64
}


impl KeyboardInputHandler {
    fn event_available(&self) -> bool{
        event::poll(Duration::from_millis(self.poll_rate)).unwrap()
    }

    pub fn get_key(&self) -> Option<KeyEvent> {
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
