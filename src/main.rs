use rand::prelude::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, style};

use std::f64::consts::PI;
use std::io::{stdin, stdout, Write};
use std::{cmp, thread, time};

enum Material {
    Sand,
    Water,
}

struct Point {
    x: u16,
    y: u16,
    material: Material,
}

impl Point {
    fn new(material: Material, x: u16, y: u16) -> Point {
        Point { x, y, material }
    }
}

struct Adjacent {
    //   w: bool,
    sw: bool,
    s: bool,
    se: bool,
    //    e: bool
}

// TODO add a `match` block to handle different material types
fn drop_or_settle(point: &Point, adjacent: Adjacent, frame: u16) -> Option<Point> {
    let is_even = (frame & 1) == 0;

    // drop
    // s
    if adjacent.s {
        return Some(Point {
            x: point.x,
            y: point.y + 1,
            material: point.material,
        });
    }

    // settle
    if adjacent.sw && adjacent.se {
        if is_even {
            // sw
            return Some(Point {
                x: point.x - 1,
                y: point.y + 1,
                material: point.material,
            });
        }
        // se
        return Some(Point {
            x: point.x + 1,
            y: point.y + 1,
            material: point.material,
        });
    }
    // sw
    if adjacent.sw {
        return Some(Point {
            x: point.x - 1,
            y: point.y + 1,
            material: point.material,
        });
    }
    // se
    if adjacent.se {
        return Some(Point {
            x: point.x + 1,
            y: point.y + 1,
            material: point.material,
        });
    }

    // noop
    return None;
}

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();
    let stdin = stdin();

    let (width, height) = terminal_width_height();
    let shortest = cmp::min(width, height);
    let area = width * height;

    const FPS: u64 = 8;
    let delay = time::Duration::from_millis(1000 / FPS);
    let max_frames = 99;
    let mut frames = 0;

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // array of materials
    let mut vec: Vec<Point> = Vec::with_capacity(area as usize);
    // TODO add a single-cell border around the screen to avoid the need to boundary-check
    // --which would also help the coordinates of termion being 1,1-origin

    // populate screen
    vec.push(Point::new(Material::Water, 1, 1));
    vec.push(Point::new(Material::Water, width - 1, 1));

    let mut rng = rand::thread_rng();
    let radius = (shortest / 3) as f64;
    let circle_area = (PI * radius * radius) as u16;
    for _ in 0..circle_area {
        let a = rng.gen::<f64>() * 2.0 * PI;
        let r = radius * (rng.gen::<f64>()).sqrt();
        let x = r * a.cos();
        let y = r * a.sin();
        let x: u16 = (x + ((width / 2) as f64)) as u16;
        let y: u16 = (y + ((height / 2) as f64)) as u16;

        if vec.iter().position(|r| r.x == x && r.y == y).is_none() {
            vec.push(Point::new(Material::Sand, x, y));
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
                Adjacent {
                    // TODO water settling needs to be context-aware
                    //                    w: match vec[i].material {
                    //                        Material::Water => {
                    //                            vec[i].x > 1 && vec.iter()
                    //                            .position(|r| r.x == vec[i].x - 1 && r.y == vec[i].y)
                    //                            .is_none()
                    //                        }
                    //                        _ =>false
                    //                    },
                    sw: vec[i].x > 1
                        && vec[i].y < height
                        && vec
                            .iter()
                            .position(|r| r.x == vec[i].x - 1 && r.y == vec[i].y + 1)
                            .is_none(),
                    s: vec[i].y < height
                        && vec
                            .iter()
                            .position(|r| r.x == vec[i].x && r.y == vec[i].y + 1)
                            .is_none(),
                    se: vec[i].x < width
                        && vec[i].y < height
                        && vec
                            .iter()
                            .position(|r| r.x == vec[i].x + 1 && r.y == vec[i].y + 1)
                            .is_none(),
                    // TODO water settling needs to be context-aware
                    //                    e: match vec[i].material {
                    //                        Material::Water => {
                    //                            vec[i].x < width && vec.iter()
                    //                            .position(|r| r.x == vec[i].x + 1 && r.y == vec[i].y)
                    //                            .is_none()
                    //                        }
                    //                        _ =>false
                    //                    },
                },
                frames,
            );

            match new_point {
                Some(p) => {
                    moves += 1;
                    // erase glyph before move
                    // TODO only erase if this position will be empty
                    write!(stdout, "{}{}", cursor::Goto(vec[i].x, vec[i].y), " ").unwrap();
                    vec[i] = p;
                    // write glyph
                    match vec[i].material {
                        Material::Sand => {
                            write!(stdout, "{}{}", cursor::Goto(vec[i].x, vec[i].y), "■").unwrap()
                        }
                        _ => write!(stdout, "{}{}", cursor::Goto(vec[i].x, vec[i].y), "X").unwrap(),
                    }
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

    write!(stdout, "{}", style::Reset).unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => write!(stdout, "{}Press 'q' to exit", cursor::Goto(1, 1),).unwrap(),
        }
        stdout.flush().unwrap();
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
