// inspiration from:
// https://github.com/fdehau/tui-rs/blob/master/src/backend/termion.rs

use std::io::Write;
use termion::{clear, cursor, style};

pub struct Term<W>
where
    W: Write,
{
    stdout: W,
}

impl<W> Term<W>
where
    W: Write,
{
    pub fn new(stdout: W) -> Term<W> {
        Term { stdout }
    }

    pub fn size(&self) -> (u16, u16) {
        termion::terminal_size().expect("Unable to get terminal size")
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn restore(&mut self) {
        self.reset();
        write!(
            self.stdout,
            "{}{}{}",
            termion::clear::All,
            cursor::Goto(1, 1),
            termion::cursor::Show
        )
        .expect("Unable to restore terminal");
    }

    fn reset(&mut self) {
        write!(self.stdout, "{}", style::Reset).unwrap();
    }
}
