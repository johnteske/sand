extern crate termion;

use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use std::io::{stdout, Write};
use std::{thread, time};

mod material;
use material::Material;

struct Point {
    x: u16,
    y: u16,
}

struct Sand {
    point: Point,
    glyph: &'static str,
}

impl Material for Sand {
    fn new(x: u16, y: u16) -> Sand {
        Sand {
            point: Point { x, y },
            glyph: "█",
        }
    }

    // if space below is empty
    fn drop(&mut self) {
        self.point.y += 1;
    }

    // if space below and diagonal
    // always check both directions but make the choice based on even/odd frame
    fn settle(&mut self) {
        self.point.y += 1;
        self.point.x += 1; // or -=
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
    vec.push(Sand::new(1, 2));
    vec.push(Sand::new(3, 1));

    fn xyToIndex(width: u16, x: u16, y: u16) -> u16 {
        return (x & (width - 1)) + (y & (width - 1)) * width;
    }

    loop {
        if frames >= max_frames {
            break;
        }

        // clear screen between frames
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        //        // consider gravity (drop) first,
        //        // sort by y position to calculate bottom rows first
        //        vec.sort_unstable_by(|a, b|
        //            xyToIndex(width, b.point.x, b.point.y).cmp(
        //            &xyToIndex(width, a.point.x, a.point.y))
        //        );

        for i in 0..vec.len() {
            // calculate drop
            let index_below = vec
                .iter()
                .position(|r| r.point.x == vec[i].point.x && r.point.y == vec[i].point.y + 1);
            if vec[i].point.y < height && index_below.is_none() {
                vec[i].drop();
            }

            //m.settle();

            // write glyph
            write!(
                stdout,
                "{}{}",
                cursor::Goto(vec[i].point.x, vec[i].point.y),
                vec[i].glyph
            )
            .unwrap();
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
