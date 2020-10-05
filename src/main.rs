extern crate termion;

use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    // let termsize = termion::terminal_size().ok();
    // let termwidth = termsize.map(|(w,_)| w - 2);
    // let termheight = termsize.map(|(_,h)| h - 2);

    const BLOCK: &'static str = "▒";
    let max_frames = 30;
    let delay = time::Duration::from_millis(33);

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    let mut count = 0;
    loop {
        write!(stdout, "{}", cursor::Goto(count, 1)).unwrap();
        write!(stdout, "{}", BLOCK).unwrap();

        // increment
        stdout.flush().unwrap();
        count += 1;
        thread::sleep(delay);
        if count >= max_frames {
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show)
        //write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show)
        .expect("Unable to restore terminal");
}
