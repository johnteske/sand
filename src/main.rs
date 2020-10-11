use rand::prelude::*;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor};

use std::f64::consts::PI;
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

fn drop_or_settle(point: &Point, points_below: Vec<bool>, frame: u16) -> Option<Point> {
    let is_even = (frame & 1) == 0;

    // drop
    // s
    if points_below[1] {
        return Some(Point {
            x: point.x,
            y: point.y + 1,
        });
    }

    // settle
    if points_below[0] && points_below[2] {
        if is_even {
            // sw
            return Some(Point {
                x: point.x - 1,
                y: point.y + 1,
            });
        }
        // se
        return Some(Point {
            x: point.x + 1,
            y: point.y + 1,
        });
    }
    // sw
    if points_below[0] {
        return Some(Point {
            x: point.x - 1,
            y: point.y + 1,
        });
    }
    // se
    if points_below[2] {
        return Some(Point {
            x: point.x + 1,
            y: point.y + 1,
        });
    }

    // noop
    return None;
}

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let (width, height) = terminal_width_height();
    let area = width * height;

    const FPS: u64 = 8;
    let delay = time::Duration::from_millis(1000 / FPS);
    let max_frames = 99;
    let mut frames = 0;

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    write!(stdout, "{}", color::Fg(color::Yellow)).unwrap();

    // array of materials
    let mut vec: Vec<Point> = Vec::with_capacity(area as usize);
    // TODO add a single-cell border around the screen to avoid the need to boundary-check
    // --which would also help the coordinates of termion being 1,1-origin

    // populate screen
    // TODO this looks like it streams from the center of the screen,
    // but that also may be an issue with the sorting
    let mut rng = rand::thread_rng();
    for _ in 1..99 {
        let a = rng.gen::<f64>() * 2.0 * PI;
        let r = ((height / 3) as f64) * (rng.gen::<f64>()).sqrt();
        let x: u16 = (r * a.cos()) as u16;
        let y: u16 = (r * a.sin()) as u16;
        // do not add point if exists already
        if vec.iter().position(|r| r.x == x && r.y == y).is_none() {
            vec.push(Point::new(x + (width / 2), y + (height / 2) - (height / 3)));
        }
    }

    let mut moves = 1; // how many moves were made
    loop {
        // exit if nothing moved or
        // max_frames has been reached
        if moves == 0 || frames >= max_frames {
            break;
        }

        moves = 0;

        // TODO sort by y
        vec.sort_by(|a, b| b.y.partial_cmp(&a.y).unwrap());
        for i in 0..vec.len() {
            let new_point = drop_or_settle(
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

            match new_point {
                Some(p) => {
                    moves += 1;
                    // erase glyph before move
                    write!(stdout, "{}{}", cursor::Goto(vec[i].x, vec[i].y), " ").unwrap();
                    vec[i] = p;
                    // write glyph
                    write!(stdout, "{}{}", cursor::Goto(vec[i].x, vec[i].y), "■").unwrap();
                }
                None => {}
            }
        }

        // write to stdout
        stdout.flush().unwrap();

        // wait
        thread::sleep(delay);

        // increment
        frames += 1;
    }

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        cursor::Goto(1, 1),
        termion::cursor::Show
    )
    .expect("Unable to restore terminal");
}

fn terminal_width_height() -> (u16, u16) {
    termion::terminal_size().expect("Unable to get terminal size")
}
