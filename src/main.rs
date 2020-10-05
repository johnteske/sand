extern crate termion;

use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use std::io::{stdout, Write};
use std::{thread, time};

enum Material {
    Sand,
}

struct Pixel {
    position: usize,
    material: Material,
}

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let (width, height) = terminal_width_height();

    const BLOCK: &'static str = "▒";
    let max_frames = width; // TODO for demo purposes only
    let delay = time::Duration::from_millis(33);

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    let mut count = 0;
    loop {
        write!(stdout, "{}", cursor::Goto(count, 1)).unwrap();
        write!(stdout, "{}", BLOCK).unwrap();

        // increment
        stdout.flush().unwrap();
        thread::sleep(delay);
        count += 1;
        if count >= max_frames {
            break;
        }
    }

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show)
        .expect("Unable to restore terminal");
}

fn terminal_width_height() -> (u16, u16) {
    termion::terminal_size().expect("Unable to get terminal size")
}
