//! Some funcions to write reusable UI components
//!
pub fn is_window_docked() -> bool {
    unsafe { imgui::sys::igIsWindowDocked() }
}

pub fn docked_window<F>(
    ui: &imgui::Ui,
    gdb: &crate::debugger::DebuggerState,
    name: &str,
    dockspace: u32,
    mut f: F,
) where
    F: FnMut(&imgui::Ui, &crate::debugger::DebuggerState),
{
    imgui::Window::new(&imgui::ImString::new(name))
        .resizable(true)
        .size([150f32, 300f32], imgui::Condition::Appearing)
        .build(&ui, || {
            if !is_window_docked() && dockspace != 0 {
                unsafe {
                    imgui::sys::igDockBuilderDockWindow(
                        imgui::ImString::new(name).as_ptr(),
                        dockspace,
                    )
                }
            }

            f(ui, gdb);
        });
}

pub fn floating_window<F>(
    ui: &imgui::Ui,
    gdb: &crate::debugger::DebuggerState,
    name: &str,
    mut f: F,
) where
    F: FnMut(&imgui::Ui, &crate::debugger::DebuggerState),
{
    imgui::Window::new(&imgui::ImString::new(name))
        .resizable(true)
        .size([150f32, 300f32], imgui::Condition::Appearing)
        .build(&ui, || {
            f(ui, gdb);
        });
}
