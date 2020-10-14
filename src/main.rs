use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, style};

use std::io::{stdin, stdout, Write};
use std::{thread, time};

mod material;
use material::Material;

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();
    let stdin = stdin();

    let (width, height) = terminal_width_height();
    let area = width * height;

    const FPS: u64 = 8;
    let delay = time::Duration::from_millis(1000 / FPS);
    let max_frames = 99;
    let mut frames = 0;

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // array of materials
    let mut vec: Vec<Box<dyn Material>> = Vec::with_capacity(area as usize);
    // TODO add a single-cell border around the screen to avoid the need to boundary-check
    // --which would also help the coordinates of termion being 1,1-origin

    vec.push(Box::new(material::Sand::new(1, 1)));
    vec.push(Box::new(material::Water::new(2, 1)));

    let mut moves = 1; // how many moves were made
    loop {
        // exit if nothing moved or
        // max_frames has been reached
        if moves == 0 || frames >= max_frames {
            break;
        }

        moves = 0;

        for i in 0..vec.len() {
            // TODO sort by y
            vec.sort_by(|a, b| b.y().partial_cmp(&a.y()).unwrap());

            // erase glyph before move
            // TODO only erase if this position will be empty
            //write!(stdout, "{}{}", cursor::Goto(vec[i].x(), vec[i].y()), " ").unwrap();
            //vec[i].move();
            // write glyph
            write!(
                stdout,
                "{}{}",
                cursor::Goto(vec[i].x(), vec[i].y()),
                vec[i].glyph()
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
