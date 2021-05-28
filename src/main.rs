extern crate sdl2;

//TODO: Rewrite the threading code.

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::SendError;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc, Mutex};
use std::thread;
use std::{
    io::{self, Read},
    process,
    time::Duration,
};

use std::collections::HashSet;

use std::thread::sleep;

mod debugger;
mod graphics;
mod parser;
mod ui;

use graphics::build_text;

fn draw_test(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(5, 5, 5));
    canvas.clear();
}

fn start_graphics<F>(gdb_mutex: Arc<Mutex<debugger::DebuggerState>>, f: F, sender: &Sender<String>)
where
    F: Fn(),
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 1000, 950)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut font = ttf_context
        .load_font("./assets/JetBrainsMono.ttf", 20)
        .unwrap();
    let mut font_small = ttf_context
        .load_font("./assets/JetBrainsMono.ttf", 16)
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let surface = font
        .render("Hello world")
        .solid(sdl2::pixels::Color::RGB(0xff, 0xff, 0xff))
        .unwrap();

    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let sdl2::render::TextureQuery { width, height, .. } = texture.query();

    let mut rect = sdl2::rect::Rect::new(10, 5, width, height);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut prev_keys = HashSet::new();
    'running: loop {
        draw_test(&mut canvas);
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
        let keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys;

        if new_keys.contains(&Keycode::Right) {
            send_command("step\n", sender).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(50));
            send_command("-data-list-register-values d 0 1 2 3 4 5\n", sender).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(50));
            send_command(
                r#"
                -data-disassemble -s $pc -e "$pc + 20" -- 0
            "#,
                sender,
            )
            .unwrap();

            //std::thread::sleep(std::time::Duration::from_millis(50));
            //send_command("-stack-list-frames\n", sender).unwrap();
        }
        if new_keys.contains(&Keycode::Left) {
            send_command("reverse-step\n", sender).unwrap();
        }
        if new_keys.contains(&Keycode::R) {
            send_command("-data-list-register-values d 0 1 2 3 4 5\n", sender).unwrap();
        }
        if new_keys.contains(&Keycode::D) {
            send_command(
                r#"
                -data-disassemble -s $pc -e "$pc + 20" -- 0
            "#,
                sender,
            )
            .unwrap();
        }

        prev_keys = keys;

        let mut gdb = gdb_mutex.lock().unwrap();
        if let Some(str) = gdb.get_file() {
            let (t, r) = build_text(&str, &font, &texture_creator, Color::RGB(0xff, 0xff, 0xff));
            texture = t;
            println!("old rect {:?}", rect);
            println!("new rect {:?}", r);
            rect = r;

            println!("=============>New text!");
        }

        let r = Rect::new(0, 10 + (gdb.line - 1) as i32 * 27, 100, 10);

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0x40, 0, 0x40));
        canvas.fill_rect(r).unwrap();

        canvas.copy(&texture, None, Some(rect)).unwrap();

        graphics::draw_variables(&mut canvas, &gdb.variables, &font_small, &texture_creator);
        graphics::draw_regs(&mut canvas, &gdb.registers, &font_small, &texture_creator);
        graphics::draw_asm(&mut canvas, &gdb.asm, &font_small, &texture_creator);

        canvas.present();
        f();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn start_process_thread(
    child: &mut Child,
    receiver: Receiver<String>,
    gdb_mutex: Arc<Mutex<debugger::DebuggerState>>,
) {
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    // Receiving commands and sending them to GDB's stdin
    thread::spawn(move || {
        for line in receiver {
            stdin.write_all(line.as_bytes()).unwrap();
        }
    });

    // Reading and processing GDB stdout
    thread::spawn(move || {
        let mut f = BufReader::new(stdout);
        loop {
            let mut line = String::new();
            f.read_line(&mut line).unwrap();
            print!("[LINE] {}", line);

            let vals = parser::parse(&line);
            println!("[PARSER] {:#?}", &vals);

            if let Ok(v) = vals {
                // Here we try to limit the scope were we hold the mutex
                let mut gdb = gdb_mutex.lock().unwrap();
                gdb.update(&v);
            }
        }
    });
}

fn start_process(
    receiver: Receiver<String>,
    gdb_mutex: Arc<Mutex<debugger::DebuggerState>>,
) -> Child {
    let mut child = Command::new("gdb")
        .arg("--interpreter=mi3")
        .arg("./examples/a.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    start_process_thread(&mut child, receiver, gdb_mutex);
    println!("Started process: {}", child.id());

    child
}

pub fn send_command(command: &str, sender: &Sender<String>) -> Result<(), SendError<String>> {
    sender.send(String::from(command))?;

    Ok(())
}

fn main() -> Result<(), Error> {
    let (tx, rx) = channel();

    let gdb_mutex = Arc::new(Mutex::new(debugger::DebuggerState::new()));

    let mut child = start_process(rx, Arc::clone(&gdb_mutex));

    thread::sleep(std::time::Duration::from_millis(100));
    send_command("start\n", &tx).unwrap();
    thread::sleep(std::time::Duration::from_millis(100));
    //TODO: this doesn't work on windows (Test if this works on Linux)
    send_command("target record-full\n", &tx).unwrap();
    send_command("-data-list-register-names\n", &tx).unwrap();

    start_graphics(Arc::clone(&gdb_mutex), move || {}, &tx);

    child.kill()?;
    Ok(())
}
