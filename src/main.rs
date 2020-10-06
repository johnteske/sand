extern crate termion;

use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use std::io::{stdout, Write};
use std::{thread, time};

mod material;
use material::Material;

struct Sand {
    x: u16,
    y: u16,
    glyph: &'static str,
}

impl Sand {
    fn translate(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

impl Material for Sand {
    fn new(x: u16, y: u16) -> Sand {
        Sand {
            x: x,
            y: y,
            glyph: "▒",
        }
    }

    fn drop(&mut self) {
        // or use translate?
        self.y += 1;
    }
}

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let (width, height) = terminal_width_height();

    let max_frames = height;
    let delay = time::Duration::from_millis(33);

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // initialize material
    let mut test: Sand = Material::new(1, 1);

    let mut frames = 0;
    loop {
        // clear screen between frames, for now
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        //test.translate(frames, 1); //
        test.drop();

        write!(stdout, "{}", cursor::Goto(test.x, test.y)).unwrap();
        write!(stdout, "{}", test.glyph).unwrap();

        // increment
        stdout.flush().unwrap();
        thread::sleep(delay);
        frames += 1;
        // bail out
        if frames >= max_frames {
            break;
        }
    }

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show)
        .expect("Unable to restore terminal");
}

fn terminal_width_height() -> (u16, u16) {
    termion::terminal_size().expect("Unable to get terminal size")
}
