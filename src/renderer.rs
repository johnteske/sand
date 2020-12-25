use std::io::{stdout, StdoutLock, Write};
use std::sync::mpsc::{channel, Sender};
use std::{cmp, thread, time};

use termion::raw::IntoRawMode;

//mod point;
//use point::Point;
use crate::Point; // ::Point;

pub enum RenderCommand {
    Put,
    Render,
}

pub struct RenderMsg {
    pub command: RenderCommand,
    pub point: Option<Point>,
}

//struct Renderer {
//    //pub stdout: termion::raw::RawTerminal<StdoutLock<'_>>, // <Box<dyn Write>>,
//    pub stdout: termion::raw::RawTerminal<Box<dyn Write>>,
//    pub tx: Sender<String>
//}
//
//impl Renderer {
////    pub fn new() -> Renderer {
////        Renderer {}
////    }
//    pub fn put() {}
//    pub fn render() {}
//}
//
//impl Drop for Renderer {
//    fn drop(&mut self) {
//        println!("see ya");
////    write!(
////        stdout,
////        "{}{}{}{}",
////        termion::style::Reset,
////        termion::clear::All,
////        termion::cursor::Goto(1, 1),
////        termion::cursor::Show
////    )
////    .expect("Unable to restore terminal");
//
//    }
//}

// currently this assumes terminal output
//pub fn new() -> Renderer {
pub fn new() -> Sender<RenderMsg> {
    let (tx, rx) = channel::<RenderMsg>();

    //    let stdout = stdout();
    //    let stdout = stdout.lock();
    //    let mut stdout = stdout.into_raw_mode().unwrap();

    let builder = thread::Builder::new().name("renderer".into());

    // TODO on rx.recv, write
    builder
        .spawn(move || {
            let stdout = stdout();
            let stdout = stdout.lock();
            let mut stdout = stdout.into_raw_mode().unwrap();

            write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
            //write!(stdout, "{}", termion::color::Fg(termion::color::Yellow)).unwrap();

            loop {
                let msg = rx.recv().expect("could not recv");
                match msg.point {
                    Some(p) => {
                        // write!(stdout, "{}", termion::clear::All).unwrap();
                        write!(stdout, "{}■", termion::cursor::Goto(p.x(), p.y())).unwrap()
                    },
                    None => {}
                };
                //println!("recv: {}", msg.command);
                //    write!(stdout, "recv!", termion::clear::All, termion::cursor::Hide).unwrap();
                //println!("recv!");
                //let new_value = rx.recv_timeout(TIMEOUT).unwrap_or_default();
                //let mut value = state.lock().unwrap();
                //*value = new_value;
            }
        })
        .expect("could not spawn renderer thread");
    //
    // match
    //   write - set point in buffer
    //   render - render all in buffer (is this where I diff the old/new buffer?

    //Renderer { stdout: stdout, tx }
    tx
}

//fn render_loop() {
//}

// TODO impl as part of trait
pub fn dimensions() -> (u16, u16) {
    termion::terminal_size().expect("Unable to get terminal size")
}
