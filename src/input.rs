use std::io::stdin;
use termion::input::TermRead;

pub fn wait_for_any_key() {
    for c in stdin().keys() {
        match c.unwrap() {
            _ => break,
        }
    }
}
