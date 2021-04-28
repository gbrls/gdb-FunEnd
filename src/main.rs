extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc, Mutex};
use std::thread;
use std::{
    io::{self, Read},
    process,
    time::Duration,
};

use std::thread::sleep;

mod debugger;
mod parser;

fn start_graphics() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn start_process_thread(child: &mut Child, sender: Sender<String>, receiver: Receiver<String>) {
    let mut stdin = child.stdin.take().unwrap();
    let mut cnt = 0;
    let stdout = child.stdout.take().unwrap();
    thread::spawn(move || {
        let mut f = BufReader::new(stdout);

        thread::spawn(move || loop {
            let mut line = String::new();
            f.read_line(&mut line).unwrap();
            sender.send(line).unwrap();
        });

        for line in receiver {
            stdin.write_all(line.as_bytes()).unwrap();
        }
    });
}

fn start_process(sender: Sender<String>, receiver: Receiver<String>) -> Child {
    let mut child = Command::new("gdb")
        .arg("--interpreter=mi3")
        .arg("./examples/a.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    start_process_thread(&mut child, sender, receiver);
    println!("Started process: {}", child.id());

    child
}

fn start_command_thread(mutex: Mutex<Sender<String>>) {
    thread::spawn(move || loop {
        let sender = mutex.lock().unwrap();
        let mut input = String::new();
        println!("Type the next command");
        io::stdin().read_line(&mut input).unwrap();
        sender.send(input).unwrap();
    });
}

fn main() -> Result<(), Error> {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let mut child = start_process(tx1, rx2);

    start_command_thread(Mutex::new(tx2.clone()));

    for line in rx1 {
        print!("<{}>", line);
        println!("{:#?}", parser::parse(&line));
    }

    child.kill()?;
    Ok(())
}
