use std::io::Read;

use crate::parser;
use std::collections::HashMap;

/// Stores data about the current GDB execution.
pub struct DebuggerState {
    files: HashMap<String, String>,
    register_names: HashMap<String, String>,
    registers: HashMap<String, String>,
    current_file: String,
    new_file: bool,
    pub line: u32,
    pub variables: Vec<(String, String)>,
    pub asm: HashMap<String, Vec<(usize, String)>>,
    pub pc_addr: HashMap<String, usize>,
    register_order: HashMap<String, usize>,
}

fn register_ord() -> HashMap<String, usize> {
    let mut h = HashMap::new();

    let v = vec!["rax", "rbx", "rcx", "rdx", "rex", "rbp", "rsp"];

    for (i, r) in v.iter().enumerate() {
        h.insert(String::from(*r), i);
    }

    h
}

impl DebuggerState {
    pub fn new() -> DebuggerState {
        DebuggerState {
            files: HashMap::new(),
            register_names: HashMap::new(),
            registers: HashMap::new(),
            current_file: String::new(),
            new_file: false,
            line: 1,
            variables: Vec::new(),
            asm: HashMap::new(),
            pc_addr: HashMap::new(),
            register_order: register_ord(),
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

    pub fn update(&mut self, query: &crate::parser::GDBVal) {
        // Reading register mappings
        if self.register_names.is_empty() {
            DebuggerState::query_list(query, "register-names", |names| {
                println!("[REGS] #of regs = {}", names.len());
                for (i, name) in names.iter().enumerate() {
                    if let parser::GDBVal::Str(s) = name {
                        if !s.is_empty() {
                            self.register_names.insert(i.to_string(), s.to_owned());
                        }
                    }
                }
            });
            println!("Registers names: {:?}", self.register_names);
        } else {
            // If we already have the register names mapped we'll look for their values
            DebuggerState::query_list(query, "register-values", |regs| {
                for reg in regs {
                    DebuggerState::query_str(reg, "number", |number| {
                        let k = String::from(number);
                        DebuggerState::query_str(reg, "value", |value| {
                            let v = String::from(value);
                            self.registers.insert(self.register_names[&k].clone(), v);
                        });
                    });
                }
                println!("Registers vals: {:?}", self.registers);
            });
        }

        // Querying stuff from the frame like filename, funcion arguments, current line
        DebuggerState::query_val(query, "frame", |val| {
            // updating current line
            DebuggerState::query_str(val, "line", |line| {
                if let Ok(n) = line.parse::<u32>() {
                    self.line = n;
                }
            });

            // updating current file
            DebuggerState::query_str(val, "fullname", |filename| {
                self.load_file(filename).unwrap();
            });

            // reading current funcion arguments
            DebuggerState::query_list(val, "args", |args| {
                self.variables = args
                    .iter()
                    .map(|record| {
                        let mut vars = (String::from("A"), String::from("B"));

                        if let crate::parser::GDBVal::Record(rec) = record {
                            vars = (format!("{:?}", rec["name"]), format!("{:?}", rec["value"]));
                        }

                        vars
                    })
                    .collect();
            });
        });

        // Reading disassembled code
        // TODO: separate assembly code for each funcion in a hashmap
        DebuggerState::query_list(query, "asm_insns", |lines| {
            let mut first = true;
            for line in lines {
                DebuggerState::query_str(line, "func-name", |fname| {
                    DebuggerState::query_str(line, "offset", |off| {
                        if let Ok(idx) = off.parse::<usize>() {
                            if !self.asm.contains_key(fname) {
                                self.asm.insert(fname.to_string(), Vec::new());
                            }

                            if self.asm.get(fname).unwrap().len() <= idx {
                                self.asm
                                    .get_mut(fname)
                                    .unwrap()
                                    .resize(idx + 1, (0, String::new()));
                            }

                            DebuggerState::query_str(line, "address", |addr| {
                                let addr_usize = from_hex(addr);
                                if !self.pc_addr.contains_key(fname) || first {
                                    self.pc_addr.insert(fname.to_string(), addr_usize);
                                }

                                let cur_line = *self.pc_addr.get(fname).unwrap();
                                if !first {
                                    self.pc_addr.insert(
                                        fname.to_string(),
                                        std::cmp::min(cur_line, addr_usize),
                                    );
                                }

                                DebuggerState::query_str(line, "inst", |i| {
                                    let v = self.asm.get_mut(fname).unwrap()[idx] =
                                        (addr_usize, i.to_string());
                                });
                            })
                        }
                    })
                });
                first = false;
            }

            println!("Instructions {:?}", self.asm);
        });

        // Reading local variables
        DebuggerState::query_list(query, "locals", |list| {
            println!("[QUERY] {:#?}", list);
            for val in list {
                DebuggerState::query_str(val, "name", |name| {
                    DebuggerState::query_str(val, "value", |value| {
                        //println!("{:?} = {:?}", name, value);
                        self.variables.push((name.to_string(), value.to_string()));
                    })
                });
            }
        })
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
    ) -> Option<HashMap<String, crate::parser::GDBVal>> {
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

    pub fn registers_ordered(&self) -> Vec<(&String, &String)> {
        let mut v = Vec::new();
        v = self
            .registers
            .iter()
            .map(|(k, _)| (self.register_order.get(k).unwrap_or(&500), k))
            .collect();
        v.sort();
        v.iter()
            .map(|(_, k)| (*k, self.registers.get(*k).unwrap()))
            .collect()
    }
}

fn from_hex(hex: &str) -> usize {
    use std::usize;
    let without_prefix = hex.trim_start_matches("0x");
    usize::from_str_radix(without_prefix, 16).unwrap()
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
