extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;
use std::os::raw::c_char;

/*
TODO:
    - Use traits to Send, Parse and Draw
    - Write a logger to use a imgui's window
 */

use imgui::im_str;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::SendError;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Instant;
use std::{
    io::{self, Read},
    process,
    time::Duration,
};

mod debugger;
mod graphics;
mod parser;
mod ui;

use graphics::build_text;
use std::cmp::max;

fn draw_test(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(5, 5, 5));
    canvas.clear();
}

pub fn is_split(id: u32) -> bool {
    unsafe {
        let node = imgui::sys::igDockBuilderGetNode(id);
        if std::ptr::null() == node {
            false
        } else {
            imgui::sys::ImGuiDockNode_IsSplitNode(node)
        }
    }
}

pub fn is_window_docked() -> bool {
    unsafe { imgui::sys::igIsWindowDocked() }
}

fn start_graphics<F>(gdb_mutex: Arc<Mutex<debugger::DebuggerState>>, f: F, sender: &Sender<String>)
where
    F: Fn(),
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 0);
    }

    let window = video_subsystem
        .window("rust-sdl2 demo", 1000, 950)
        .position_centered()
        .resizable()
        .allow_highdpi()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window
        .gl_create_context()
        .expect("Couldn't create GL context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let mut imgui = imgui::Context::create();
    imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    let mut last_frame = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut prev_keys = HashSet::new();

    let mut file_txt = String::from("no file loaded");

    'running: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) {
                continue;
            }
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
            //send_command("-data-list-register-values d 0 1 2 3 4 5\n", sender).unwrap();
            send_command(
                "-data-list-register-values x 0 1 2 3 4 5 6 7 8 9 10\n",
                sender,
            )
            .unwrap();

            std::thread::sleep(std::time::Duration::from_millis(50));
            send_command("-stack-list-locals 1", sender).unwrap();

            std::thread::sleep(std::time::Duration::from_millis(50));
            send_command(
                r#"
                -data-disassemble -s $pc -e "$pc + 20" -- 0
            "#,
                sender,
            )
            .unwrap();
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

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        imgui.io_mut().delta_time = delta_s;

        let ui = imgui.frame();
        let mut left_dock: u32 = 0;
        let mut right_dock: u32 = 0;
        let mut right_top: u32 = 0;
        let mut right_down: u32 = 0;
        let mut main_dock: u32 = 0;

        unsafe {
            main_dock = imgui::sys::igDockSpaceOverViewport(
                imgui::sys::igGetMainViewport(),
                0,
                ::std::ptr::null::<imgui::sys::ImGuiWindowClass>(),
            );
        }

        if !is_split(main_dock) {
            unsafe {
                imgui::sys::igDockBuilderSplitNode(
                    main_dock,
                    imgui::Direction::Right as i32,
                    0.3332,
                    &mut right_dock,
                    &mut left_dock,
                );
            }
        }

        if right_dock != 0 && !is_split(right_dock) {
            unsafe {
                imgui::sys::igDockBuilderSplitNode(
                    right_dock,
                    imgui::Direction::Up as i32,
                    0.5f32,
                    &mut right_top,
                    &mut right_down,
                );
            }
        }

        let mut gdb = gdb_mutex.lock().unwrap();
        if let Some(str) = gdb.get_file() {
            file_txt = str;
        }

        imgui::Window::new(im_str!("Code"))
            .resizable(true)
            .size([150f32, 300f32], imgui::Condition::Appearing)
            .build(&ui, || {
                if !is_window_docked() && left_dock != 0 {
                    unsafe {
                        imgui::sys::igDockBuilderDockWindow(im_str!("Code").as_ptr(), left_dock)
                    }
                }
                let mut x = 1.0f32;
                for (i, l) in file_txt.lines().enumerate() {
                    if (i + 1) == gdb.line as usize {
                        ui.text_colored([x, 0f32, 0f32, 1.0f32], &l);
                        x -= 0.5f32;
                    } else {
                        ui.text_colored([x, x, x, 1.0f32], &l);
                    }
                }
            });

        //ui.text_colored([1.0f32, 1.0f32, 1.0f32, 1.0f32], &file_txt);
        imgui::Window::new(im_str!("Vars"))
            .resizable(true)
            .size([150f32, 300f32], imgui::Condition::Appearing)
            .build(&ui, || {
                if !is_window_docked() && right_down != 0 {
                    unsafe {
                        imgui::sys::igDockBuilderDockWindow(im_str!("Vars").as_ptr(), right_down)
                    }
                }
                ui.columns(2, im_str!("A"), true);
                for (k, v) in &gdb.variables {
                    ui.text(k);
                    ui.next_column();
                    ui.text(v);
                    ui.next_column();
                }
            });
        //let wname = im_str!("Vars");
        //unsafe { imgui::sys::igDockBuilderDockWindow(wname.as_ptr(), imgui::sys::igGetMainViewport()); }

        imgui::Window::new(im_str!("Regs"))
            .resizable(true)
            .size([150f32, 300f32], imgui::Condition::Appearing)
            .build(&ui, || {
                if !is_window_docked() && right_top != 0 {
                    unsafe {
                        imgui::sys::igDockBuilderDockWindow(im_str!("Regs").as_ptr(), right_top)
                    }
                }
                ui.columns(2, im_str!("A"), true);
                for (k, v) in &gdb.registers_ordered() {
                    ui.text(k);
                    ui.next_column();
                    ui.text(v);
                    ui.next_column();
                }
            });

        imgui::Window::new(im_str!("Asm"))
            .resizable(true)
            .size([200f32, 200f32], imgui::Condition::Appearing)
            .build(&ui, || {
                imgui::TabBar::new(im_str!("test"))
                    .reorderable(true)
                    .build(&ui, || {
                        for (k, v) in &gdb.asm {
                            let mut s: &imgui::ImStr;
                            let mut c_str: std::ffi::CString;
                            unsafe {
                                c_str = std::ffi::CString::new(k.as_str()).unwrap();
                                s = imgui::ImStr::from_utf8_with_nul_unchecked(
                                    c_str.as_bytes_with_nul(),
                                );
                            }
                            let pc_addr = gdb.pc_addr.get(k).unwrap();
                            imgui::TabItem::new(s).build(&ui, || {
                                ui.text_colored(
                                    [0.8f32, 0.8f32, 0.2f32, 1f32],
                                    format!("{:#x}", pc_addr),
                                );
                                ui.separator();
                                ui.columns(2, im_str!("asm_col"), true);
                                let mut code = String::new();
                                for (addr, line) in v {
                                    if line.len() > 0 {
                                        if addr == pc_addr {
                                            ui.text_colored(
                                                [1f32, 0f32, 0f32, 1f32],
                                                format!("{:#x}", addr),
                                            );
                                        } else {
                                            ui.text_colored(
                                                [1f32, 1f32, 1f32, 1f32],
                                                format!("{:#x}", addr),
                                            );
                                        }
                                        ui.next_column();
                                        ui.text_colored([1f32, 1f32, 1f32, 1f32], line);
                                        ui.next_column();
                                    }
                                }
                            })
                        }
                    })
            });

        //ui.show_demo_window(&mut true);

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        imgui_sdl2.prepare_render(&ui, &window);
        renderer.render(ui);

        window.gl_swap_window();

        //::std::thread::sleep(max(
        //    Duration::from_millis(16) - last_frame.elapsed(),
        //    Duration::from_millis(0),
        //));

        //graphics::draw_variables(&mut canvas, &gdb.variables, &font_small, &texture_creator);
        //graphics::draw_regs(&mut canvas, &gdb.registers, &font_small, &texture_creator);
        //graphics::draw_asm(&mut canvas, &gdb.asm, &font_small, &texture_creator);
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
