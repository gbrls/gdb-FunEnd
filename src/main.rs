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
mod graphics;
mod parser;
mod ui;

use graphics::build_text;

fn draw_test(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(5, 5, 5));
    canvas.clear();
}

fn start_graphics<F>(gdb_mutex: Arc<Mutex<debugger::DebuggerState>>, f: F)
where
    F: Fn(),
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut font = ttf_context
        .load_font("./assets/JetBrainsMono.ttf", 20)
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
    'running: loop {
        let mut gdb = gdb_mutex.lock().unwrap();
        if let Some(str) = gdb.get_file() {
            let (t, r) = build_text(&str, &font, &texture_creator, Color::RGB(0xff, 0xff, 0xff));
            texture = t;
            println!("old rect {:?}", rect);
            println!("new rect {:?}", r);
            rect = r;

            println!("=============>New text!");
        }

        //canvas.set_draw_color(Color::RGB(5, 5, 50));
        //canvas.clear();
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
        // The rest of the game loop goes here...

        let l_str = format!("{}", gdb.line);
        let (t, mut r) = build_text(
            &l_str,
            &font,
            &texture_creator,
            Color::RGB(0xa0, 0xa0, 0xff),
        );

        r.set_x(400);
        r.set_y(10 + (gdb.line - 1) as i32 * 27);

        canvas.copy(&t, None, Some(r)).unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0xff, 0, 0xff));
        r.set_x(0);
        r.set_width(100);
        r.set_height(10);
        canvas.fill_rect(r);

        canvas.copy(&texture, None, Some(rect)).unwrap();

        graphics::draw_variables(&mut canvas, &gdb.variables, &font, &texture_creator);

        canvas.present();
        f();
        //std::thread::sleep(std::time::Duration::from_millis(10));
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
            //TODO: Parse the line here and maybe write to the GDB State
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

fn start_command_thread(rx: Receiver<String>, gdb_mutex: Arc<Mutex<debugger::DebuggerState>>) {
    thread::spawn(move || {
        for line in rx {
            let mut gdb = gdb_mutex.lock().unwrap();
            print!("<{}>", line);
            let vals = parser::parse(&line);
            println!("{:#?}", &vals);
            if let Ok(v) = vals {
                gdb.update_file(&v);
            }
        }
    });
}

fn main() -> Result<(), Error> {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let mut child = start_process(tx1, rx2);

    let gdb_mutex = Arc::new(Mutex::new(debugger::DebuggerState::new()));

    start_command_thread(rx1, Arc::clone(&gdb_mutex));

    thread::spawn(move || loop {
        let mut input = String::new();
        println!("Type the next command");
        io::stdin().read_line(&mut input).unwrap();
        tx2.send(input).unwrap();
    });

    start_graphics(Arc::clone(&gdb_mutex), move || {
        //let mut input = String::new();
        //println!("Type the next command");
        //io::stdin().read_line(&mut input).unwrap();
        //tx2.send(input).unwrap();
    });

    child.kill()?;
    Ok(())
}
