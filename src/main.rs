extern crate termion;

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

fn drop_or_settle(point: &Point, points_below: Vec<bool>) -> Point {
    // drop
    if points_below[1] {
        return Point {
            x: point.x,
            y: point.y + 1,
        };
    }

    // settle
    if points_below[2] {
        // se
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

    const FPS: u64 = 15;
    let delay = time::Duration::from_millis(1000 / FPS);
    let max_frames = height;
    let mut frames = 0;

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // array of materials
    let mut vec: Vec<Point> = Vec::with_capacity(area as usize);
    // TODO add a single-cell border around the screen to avoid the need to boundary-check
    // --which would also help the coordinates of termion being 1,1-origin
    vec.push(Point::new(1, 1));
    vec.push(Point::new(3, 1));
    vec.push(Point::new(1, 3));

    loop {
        if frames >= max_frames {
            break;
        }

        // clear screen between frames
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        // TODO sort by y
        for i in 0..vec.len() {
            // TODO check boundaries
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
            );

            // write glyph
            write!(stdout, "{}{}", cursor::Goto(vec[i].x, vec[i].y), "#").unwrap();
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
