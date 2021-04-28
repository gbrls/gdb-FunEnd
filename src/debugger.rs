use std::io::Read;

struct DebuggerState {
    files: std::collections::HashMap<String, String>,
}

impl DebuggerState {
    fn new() -> DebuggerState {
        DebuggerState {
            files: std::collections::HashMap::new(),
        }
    }

    fn load_file(&mut self, filename: &str) -> std::io::Result<()> {
        if !self.files.contains_key(filename) {
            let mut file = std::fs::File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            println!("{}", &contents);

            self.files.insert(contents, filename.to_owned());
        }
        Ok(())
    }
}

mod tests {
    #[test]
    fn load_file() {
        use super::*;
        let mut dbg = DebuggerState::new();

        dbg.load_file("C:\\\\Users\\\\gabri\\\\Programming\\\\GDB-FunEnd\\\\examples\\\\a.c")
            .unwrap();
    }
}
