extern crate termion;

use std::f64::consts::PI;

use rand::prelude::*;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use std::io::{stdout, Write};
use std::{thread, time};

struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

fn drop_or_settle(point: &Point, points_below: Vec<bool>, frame: u16) -> Point {
    let is_even = (frame & 1) == 0;

    // drop
    // s
    if points_below[1] {
        return Point {
            x: point.x,
            y: point.y + 1,
        };
    }

    // settle
    if points_below[0] && points_below[2] {
        if is_even {
            // sw
            return Point {
                x: point.x - 1,
                y: point.y + 1,
            };
        }
        // se
        return Point {
            x: point.x + 1,
            y: point.y + 1,
        };
    }
    // sw
    if points_below[0] {
        return Point {
            x: point.x - 1,
            y: point.y + 1,
        };
    }
    // se
    if points_below[2] {
        return Point {
            x: point.x + 1,
            y: point.y + 1,
        };
    }

    // noop
    return Point {
        x: point.x,
        y: point.y,
    };
}

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let (width, height) = terminal_width_height();
    let area = width * height;

    const FPS: u64 = 8;
    let delay = time::Duration::from_millis(1000 / FPS);
    let max_frames = height;
    let mut frames = 0;

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // array of materials
    let mut vec: Vec<Point> = Vec::with_capacity(area as usize);
    // TODO add a single-cell border around the screen to avoid the need to boundary-check
    // --which would also help the coordinates of termion being 1,1-origin

    // populate screen
    // TODO this looks like it streams from the center of the screen
    let mut rng = rand::thread_rng();
    for _ in 1..99 {
        let a = rng.gen::<f64>() * 2.0 * PI;
        let r = ((height / 3) as f64) * (rng.gen::<f64>()).sqrt();
        let x: u16 = (r * a.cos()) as u16;
        let y: u16 = (r * a.sin()) as u16;
        // do not add point if exists already
        if vec.iter().position(|r| r.x == x && r.y == y).is_none() {
            vec.push(Point::new(x + (width / 2), y + (height / 2)));
        }
    }

    loop {
        if frames >= max_frames {
            break;
        }

        // TODO sort by y
        for i in 0..vec.len() {
            // erase glyph before move
            // TODO only erase if moving
            write!(stdout, "{}{}", cursor::Goto(vec[i].x, vec[i].y), " ").unwrap();

            vec[i] = drop_or_settle(
                &vec[i],
                vec![
                    // sw
                    vec[i].x > 1
                        && vec[i].y < height
                        && vec
                            .iter()
                            .position(|r| r.x == vec[i].x - 1 && r.y == vec[i].y + 1)
                            .is_none(),
                    // s
                    vec[i].y < height
                        && vec
                            .iter()
                            .position(|r| r.x == vec[i].x && r.y == vec[i].y + 1)
                            .is_none(),
                    // se
                    vec[i].x < width
                        && vec[i].y < height
                        && vec
                            .iter()
                            .position(|r| r.x == vec[i].x + 1 && r.y == vec[i].y + 1)
                            .is_none(),
                ],
                frames,
            );

            // write glyph
            write!(stdout, "{}{}", cursor::Goto(vec[i].x, vec[i].y), "■").unwrap();
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
