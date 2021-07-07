extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

/*
@TODO:
    - Show line numbers!
    - Use traits to Send, Parse and Draw
    - Create a checkbox to enable debugging the parser, queries, etc;
    - Write a logger to use a imgui window
 */

use imgui::im_str;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

use ui::is_window_docked;

fn send_commands(sender: &Sender<String>, commands: &[&str], time: u64) {
    for command in commands {
        send_command(command, &sender).unwrap();
        sleep(Duration::from_millis(time));
    }
}

pub fn send_command(command: &str, sender: &Sender<String>) -> Result<(), SendError<String>> {
    sender.send(String::from(command))?;

    Ok(())
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

const STEP_COMMANDS: [&str; 5] = [
    "step\n",
    "-data-list-register-values x\n",
    "-stack-list-locals 1\n",
    r#" -data-disassemble -s $pc -e "$pc + 20" -- 0 
                "#,
    r#" -data-read-memory $sp x 1 1 128
                "#,
];

const STARTUP_COMMANDS: [&str; 3] = [
    "target remote :1234\n",
    "break main\n",

    //"start\n",
    //"target record-full\n",
    "-data-list-register-names\n",
];

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

    let mut path = std::path::PathBuf::new();
    path.push("imgui");
    path.set_extension("ini");

    //imgui.set_ini_filename(Some(path));
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    let mut last_frame = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut prev_keys = HashSet::new();

    let mut file_txt = String::from("no file loaded");

    let mut input_buf = imgui::ImString::new("type something here");

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

        // Call step commands
        if new_keys.contains(&Keycode::Right) {
            send_commands(sender, &STEP_COMMANDS, 50);
        }
        if new_keys.contains(&Keycode::Left) {
            send_command("reverse-step\n", sender).unwrap();
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
        let mut left_top: u32 = 0;
        let mut left_down: u32 = 0;
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
                    0.3f32,
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

        if left_dock != 0 && !is_split(left_dock) {
            unsafe {
                imgui::sys::igDockBuilderSplitNode(
                    left_dock,
                    imgui::Direction::Up as i32,
                    0.65f32,
                    &mut left_top,
                    &mut left_down,
                );
            }
        }

        let mut gdb = gdb_mutex.lock().unwrap();
        if let Some(str) = gdb.get_file() {
            file_txt = str;
        }

        ui::docked_window(&ui, &mut gdb, "Code", left_top, |ui, gdb| {
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

        ui::docked_window(&ui, &mut gdb, "Vars", right_down, |ui, gdb| {
            ui.columns(2, im_str!("A"), true);
            for (k, v) in &gdb.variables {
                ui.text(k);
                ui.next_column();
                ui.text(v);
                ui.next_column();
            }
        });

        ui::docked_window(&ui, &mut gdb, "Regs", right_top, |ui, gdb| {
            ui.columns(2, im_str!("A"), true);
            for (k, v) in &gdb.registers_ordered() {
                ui.text(k);
                ui.next_column();
                ui.text(v);
                ui.next_column();
            }
        });

        ui::docked_window(&ui, &mut gdb, "Asm", left_down, |ui, gdb| {
            {
                imgui::TabBar::new(im_str!("test"))
                    .reorderable(true)
                    .build(&ui, || {
                        for (k, v) in &gdb.asm {
                            let s: &imgui::ImStr;
                            let c_str: std::ffi::CString;
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
            }
        });

        ui::docked_window(&ui, &gdb, "Console", left_down, |ui, gdb| {
            ui.text_colored([1f32, 1f32, 1f32, 1f32], &gdb.console_output);
            if imgui::InputText::new(ui, im_str!(""), &mut input_buf)
                .enter_returns_true(true)
                .build()
            {
                let mut cmd = String::from(input_buf.to_str());
                cmd.push('\n');
                send_command(&cmd, &sender).unwrap();
                input_buf.clear();
            }
        });

        ui::docked_window(&ui, &gdb, "memory", right_down, |ui, gdb| {
            let (addr, mem) = &gdb.memory;
            let mut addr = *addr;
            let mut s = format!("{:#08x}  ", addr);
            let mut col = 0.2f32;
            for (i, val) in mem.iter().enumerate() {
                if *val != 0u64 {
                    col = 1f32;
                }
                s.push_str(&format!("{:02x}", val));
                s.push(' ');
                addr += 1;

                if (i + 1) % 8 == 0 {
                    ui.text_colored([col, col, col, 1f32], &s);
                    // cleaning the string for the next line
                    s = format!("{:#08x}  ", addr);
                    col = 0.2f32;
                }
            }
            //@Error maybe some values won't be rendered here
        });

        //ui.show_demo_window(&mut true);

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        imgui_sdl2.prepare_render(&ui, &window);
        renderer.render(ui);

        window.gl_swap_window();
    }
}

fn start_process_thread(
    child: &mut Child,
    receiver: Receiver<String>,
    gdb_mutex: Arc<Mutex<debugger::DebuggerState>>,
) {
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    use crate::debugger::DebuggerState;

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

            let gdb: &mut DebuggerState = &mut *gdb_mutex.lock().unwrap();
            let vals = parser::parse(&line, gdb);
            println!("[PARSER] {:#?}", &vals);

            if let Ok(v) = vals {
                // Here we try to limit the scope were we hold the mutex
                gdb.update(&v);
            }
        }
    });
}

fn start_process(
    receiver: Receiver<String>,
    gdb_mutex: Arc<Mutex<debugger::DebuggerState>>,
) -> Child {
    let mut child = Command::new("gdb-multiarch")
        .arg("--interpreter=mi3")
        .arg("./arm-examples/a")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    start_process_thread(&mut child, receiver, gdb_mutex);
    println!("Started process: {}", child.id());

    child
}

fn main() -> Result<(), Error> {
    let (tx, rx) = channel();

    let gdb_mutex = Arc::new(Mutex::new(debugger::DebuggerState::new()));

    // qemu-aarch64 -g 1234 ./a
    let mut qemu = Command::new("qemu-aarch64")
        .arg("-g")
        .arg("1234")
        .arg("arm-examples/a")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start qemu");

    let stdout = qemu.stdout.take().unwrap();
    thread::spawn(move || {
        let mut f = BufReader::new(stdout);
        loop {
            let mut line = String::new();
            f.read_line(&mut line).unwrap();
            print!("[QEMU] {}", line);
        }
    });

    let mut child = start_process(rx, Arc::clone(&gdb_mutex));

    send_commands(&tx, &STARTUP_COMMANDS, 100);

    start_graphics(Arc::clone(&gdb_mutex), move || {}, &tx);

    qemu.kill()?;
    child.kill()?;

    Ok(())
}
