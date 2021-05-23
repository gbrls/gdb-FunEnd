use std::io::Read;

use crate::debugger;

/// Stores data about the current GDB execution.
pub struct DebuggerState {
    files: std::collections::HashMap<String, String>,
    current_file: String,
    new_file: bool,
    pub line: u32,
    pub variables: Vec<(String, String)>,
}

impl DebuggerState {
    pub fn new() -> DebuggerState {
        DebuggerState {
            files: std::collections::HashMap::new(),
            current_file: String::new(),
            new_file: false,
            line: 1,
            variables: Vec::new(),
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
        DebuggerState::query_val(query, "frame", |val| {
            DebuggerState::query_str(val, "line", |line| {
                if let Ok(n) = line.parse::<u32>() {
                    self.line = n;
                }
            });

            DebuggerState::query_str(val, "fullname", |filename| {
                self.load_file(filename).unwrap();
            });

            DebuggerState::query_list(val, "args", |args| {
                self.variables = args
                    .iter()
                    .map(|record| {
                        let mut vars = (String::from("A"), String::from("B"));

                        if let crate::parser::GDBVal::Record(rec) = record {
                            vars = (format!("{:?}", rec["name"]), format!("{:?}", rec["value"]));
                        }

                        return vars;
                    })
                    .collect();
            });
        });
    }

    // TODO: Write a single funtion to handle the multiple variants,
    pub fn query_val<F>(query: &crate::parser::GDBVal, key: &str, mut f: F)
    where
        F: FnMut(&crate::parser::GDBVal),
    {
        use crate::parser::GDBVal;
        let mut found = false;
        if let GDBVal::Record(record) = query {
            if record.contains_key(key) {
                f(&record[key]);
                found = true;
            }
        }

        if !found {
            println!("[QUERY] Expected {:?} found {:?}", key, query);
        }
    }

    pub fn query_str<F>(query: &crate::parser::GDBVal, key: &str, mut f: F)
    where
        F: FnMut(&str),
    {
        use crate::parser::GDBVal;
        let mut found = false;
        if let GDBVal::Record(record) = query {
            if record.contains_key(key) {
                if let GDBVal::Str(s) = &record[key] {
                    f(s);
                    found = true;
                }
            }
        }

        if !found {
            println!("[QUERY] Expected {:?} found {:?}", key, query);
        }
    }

    pub fn query_list<F>(query: &crate::parser::GDBVal, key: &str, mut f: F)
    where
        F: FnMut(&Vec<crate::parser::GDBVal>),
    {
        use crate::parser::GDBVal;
        let mut found = false;
        if let GDBVal::Record(record) = query {
            if record.contains_key(key) {
                if let GDBVal::List(l) = &record[key] {
                    f(l);
                    found = true;
                }
            }
        }

        if !found {
            println!("[QUERY] Expected {:?} found {:?}", key, query);
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
