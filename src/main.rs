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

impl Material for Sand {
    fn new(x: u16, y: u16) -> Sand {
        Sand {
            x: x,
            y: y,
            glyph: "▒",
        }
    }

    // if space below is empty
    fn drop(&mut self) {
        self.y += 1;
    }

    // if space below and diagonal
    // always check both directions but make the choice based on even/odd frame
    fn settle(&mut self) {
        self.y += 1;
        self.x += 1;
        //self.x -= 1;
    }
}

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let (width, height) = terminal_width_height();
    let area = width * height;

    const FPS: u64 = 15;
    let delay = time::Duration::from_millis(1000 / FPS);
    let max_frames = height;
    let mut frames = 0;

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // array of materials
    let mut vec: Vec<Sand> = Vec::with_capacity(area as usize);
    // TODO add a single-cell border around the screen to avoid the need to boundary-check
    // --which would also help the coordinates of termion being 1,1-origin
    vec.push(Sand::new(1, 1));
    vec.push(Sand::new(3, 1));

    loop {
        if frames >= max_frames {
            break;
        }

        // clear screen between frames
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        // TODO sort by y position
        for m in vec.iter_mut() {
            // calculate material change
            m.drop();
            //m.settle();

            // write glyph
            write!(stdout, "{}{}", cursor::Goto(m.x, m.y), m.glyph).unwrap();
        }

        // write to stdout
        stdout.flush().unwrap();

        // wait
        thread::sleep(delay);

        // increment
        frames += 1;
    }

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show)
        .expect("Unable to restore terminal");
}

fn terminal_width_height() -> (u16, u16) {
    termion::terminal_size().expect("Unable to get terminal size")
}
