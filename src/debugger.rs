use std::io::Read;

/// Stores data about the current GDB execution.
pub struct DebuggerState {
    files: std::collections::HashMap<String, String>,
    current_file: String,
    new_file: bool,
    pub line: u32,
}

impl DebuggerState {
    pub fn new() -> DebuggerState {
        DebuggerState {
            files: std::collections::HashMap::new(),
            current_file: String::new(),
            new_file: false,
            line: 1,
        }
    }

    pub fn get_file(&mut self) -> Option<String> {
        if self.new_file {
            self.new_file = false;
            Some(self.files[&self.current_file].clone())
        } else {
            None
        }
    }

    fn load_file(&mut self, filename: &str) -> std::io::Result<()> {
        if !self.files.contains_key(filename) {
            let mut file = std::fs::File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            println!("CONTENTS {}", &contents);

            self.files.insert(filename.to_owned(), contents);
        }

        if self.current_file != filename {
            self.new_file = true;
        }

        self.current_file = filename.to_owned();
        Ok(())
    }

    pub fn update_file(&mut self, query: &crate::parser::GDBVal) {
        use crate::parser::GDBVal;
        if let GDBVal::Record(record) = query {
            let q = String::from("frame");
            if !record.contains_key(&q) {
                return;
            }
            if let GDBVal::Record(frame) = &record[&q] {
                let q = String::from("line");
                if frame.contains_key(&q) {
                    if let GDBVal::Str(line) = &frame[&q] {
                        if let Ok(n) = line.parse::<u32>() {
                            self.line = n;
                        }
                    }
                }
                let q = String::from("fullname");
                if !frame.contains_key(&q) {
                    return;
                }
                if let GDBVal::Str(filename) = &frame[&q] {
                    self.load_file(filename).unwrap();
                    return;
                }
            }
        }
        println!("No file open");
    }

    pub fn query_str<F>(query: &crate::parser::GDBVal, key: &str, mut f: F) where
    F: FnMut(&crate::parser::GDBVal) {

        use crate::parser::GDBVal;
        if let GDBVal::Record(record) = query {
            if record.contains_key(key) {
                f(&record[key]);
            }
        }
    }

    /// We will use this funcion to store the frame and query it's values
    fn frame(
        &self,
        query: &crate::parser::GDBVal,
    ) -> Option<std::collections::HashMap<String, crate::parser::GDBVal>> {
        use crate::parser::GDBVal;
        if let GDBVal::Record(record) = query {
            let q = String::from("frame");
            if !record.contains_key(&q) {
                return None;
            }
            if let GDBVal::Record(frame) = &record[&q] {
                return Some(frame.clone());
            }
        }

        None
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
