use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{stdin, stdout};
use std::{thread, time};

mod material;
use material::{Material, Sand};

mod term;

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let stdout = stdout.into_raw_mode().unwrap();

    let stdin = stdin();

    let mut t = term::Term::new(stdout);

    let (width, height) = t.size();
    let area = width * height;

    const FPS: u64 = 8;
    let delay = time::Duration::from_millis(1000 / FPS);
    let max_frames = 99;
    let mut frames = 0;

    t.clear();

    // array of materials
    let mut vec: Vec<Material> = Vec::with_capacity(area as usize);

    // populate screen
    vec.push(Material::Sand(Sand { x: 1, y: 1 }));

    let mut moves = 1; // how many moves were made
    loop {
        // exit if nothing moved or
        // max_frames has been reached
        if moves == 0 || frames >= max_frames {
            break;
        }

        // moves = 0;
        moves += 0;

        // write to stdout
        t.flush().unwrap();

        // wait
        thread::sleep(delay);

        // increment
        frames += 1;
    }

    t.reset();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => t.wr("Press 'q' to exit"),
        }
        t.flush().unwrap();
    }

    t.restore();
}
