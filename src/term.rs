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

    pub fn clear(&mut self) {
        write!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    }

    pub fn reset(&mut self) {
        write!(self.stdout, "{}", style::Reset).unwrap();
    }

    pub fn restore(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            termion::clear::All,
            cursor::Goto(1, 1),
            termion::cursor::Show
        )
        .expect("Unable to restore terminal");
    }

    pub fn size(&self) -> (u16, u16) {
        termion::terminal_size().expect("Unable to get terminal size")
    }

    // should this be private?
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
    pub fn wr(&mut self, s: &str) {
        write!(self.stdout, "{}{}", s, cursor::Goto(1, 1)).unwrap();
    }
}

//impl<W> Write for Term<W>
//where
//    W: Write,
//{
//    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//                self.stdout.write(buf)
//                    }
//
//    fn flush(&mut self) -> std::io::Result<()> {
//                self.stdout.flush()
//                    }
//
//}
