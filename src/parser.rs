use std::{collections::HashMap, ops::Index};
#[derive(Debug, Clone)]
pub struct ParseError {
    cause: String,
}

/// Tokenizes GBD Strings
/// format reference https://sourceware.org/gdb/onlinedocs/gdb/GDB_002fMI-Output-Syntax.html#GDB_002fMI-Output-Syntax
#[derive(Debug, PartialEq, Clone)]
pub enum GDBToken {
    Comma,
    OpenPb,
    ClosePb,
    OpenB,
    CloseB,
    Eq,

    Number(u64),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum GDBVal {
    Record(HashMap<String, GDBVal>),
    List(Vec<GDBVal>),
    Number(u64),
    Str(String),
}

pub fn parse(str: &str) -> Result<GDBVal, ParseError> {
    let tokens = tokenize(str);
    println!("{:?}", &tokens);
    let mut p = Parser::new(tokens);

    p.parse()
}

fn parse_helper(str: &str) {
    let tokens = tokenize(str);
}

pub struct Parser {
    tokens: Vec<GDBToken>,
    vals: Vec<HashMap<String, GDBVal>>,
    cur: usize,
}

impl Parser {
    fn new(tokens: Vec<GDBToken>) -> Parser {
        Parser {
            tokens,
            cur: 0,
            vals: Vec::new(),
        }
    }

    fn cur_token(&self) -> GDBToken {
        self.tokens[self.cur].clone()
    }

    fn consume(&mut self) {
        self.cur += 1;
    }

    fn expect(&mut self, token: GDBToken) -> Result<GDBToken, ParseError> {
        let t = self.cur_token();
        if t != token {
            Err(ParseError {
                cause: format!("[token {}]: Expected {:?}, found {:?}", self.cur, token, t),
            })
        } else {
            self.consume();
            Ok(t)
        }
    }

    fn parse(&mut self) -> Result<GDBVal, ParseError> {
        use GDBToken::*;

        let p = match self.cur_token() {
            Str(s) => {
                self.consume();
                GDBVal::Str(s.to_owned())
            }
            OpenB => {
                let mut v = Vec::new();
                self.consume();
                while self.cur_token() != CloseB {
                    if v.len() > 0 {
                        self.expect(Comma)?;
                    }
                    v.push(self.parse()?);
                }
                self.expect(CloseB)?;
                GDBVal::List(v)
            }

            OpenPb => {
                let mut mp = HashMap::new();
                self.consume();

                while let GDBToken::Str(str) = self.cur_token() {
                    self.consume();

                    if self.cur_token() == ClosePb {
                        // something like (gdb)
                        continue;
                    }
                    if self.cur_token() == Comma {
                        // something like *ignoreme,a=10
                        self.consume();
                        continue;
                    }

                    self.expect(Eq)?;
                    let val = self.parse()?;

                    mp.insert(str, val);

                    if self.cur_token() == Comma {
                        self.consume();
                    }
                }

                self.expect(ClosePb)?;

                GDBVal::Record(mp)
            }
            //OpenPb => {}
            x => panic!("Token {:?} not expected", x),
        };

        Ok(p)
    }
}

// really ugly code, I know...
fn number_or_string(input: &mut Vec<char>, str: bool) -> GDBToken {
    let mut s = String::new();

    while !input.is_empty() {
        let c = *input.last().unwrap();
        if !str && (c == '=' || c == ',' || c == ' ') {
            break;
        }

        input.pop();

        if c == '\"' {
            break;
        }

        s.push(c);
    }

    //println!("last = {:?}", input.last().unwrap());

    //TODO: convert to number if it's the case.
    GDBToken::Str(s)
}

pub fn tokenize(input: &str) -> Vec<GDBToken> {
    use GDBToken::*;
    let mut tokens = vec![OpenPb];
    let mut chars: Vec<char> = input.as_bytes().iter().rev().map(|x| *x as char).collect();

    //println!("{:?}", chars);
    let mut ignore = false;

    while !chars.is_empty() {
        let c = *chars.last().unwrap();
        chars.pop();

        if c == '~' {
            ignore = true;
        }

        if ignore {
            if c == '\n' {
                ignore = false;
            }

            continue;
        }

        let token = match c {
            '[' => Some(OpenB),
            ']' => Some(CloseB),
            '{' => Some(OpenPb),
            '}' => Some(ClosePb),

            ',' => Some(Comma),
            '=' => Some(Eq),

            '\"' => Some(number_or_string(&mut chars, true)),

            c if c.is_alphabetic() => {
                chars.push(c);
                Some(number_or_string(&mut chars, false))
            }

            _ => None,
        };

        if let Some(t) = token {
            tokens.push(t);
        }
    }

    tokens.push(ClosePb);
    tokens
}

#[cfg(test)]
mod tests {
    #[test]
    fn tk_test() {
        use super::*;
        println!("{:?}", tokenize("ola=\"voce\",,="));
        println!("{:?}", tokenize("{address=\"0x000107bc\",func-name=\"main\",offset=\"0\", inst=\"save  %sp, -112, %sp\"},"));
    }

    #[test]
    fn test_parse() {
        use super::*;
        let str = r#" registers=["0","1","2","4","5","6","7","8","9", "10","11","13","14","15","16","17","18","19","20","21","22","23", "24","25","26","27","28","30","31","64","65","66","67","69"] "#;

        println!("{:#?}", parse(str));

        let str = r#" register-names=["r0","r1","r2","r3","r4","r5","r6","r7",
            "r8","r9","r10","r11","r12","r13","r14","r15","r16","r17","r18",
            "r19","r20","r21","r22","r23","r24","r25","r26","r27","r28","r29",
            "r30","r31","f0","f1","f2","f3","f4","f5","f6","f7","f8","f9",
            "f10","f11","f12","f13","f14","f15","f16","f17","f18","f19","f20",
            "f21","f22","f23","f24","f25","f26","f27","f28","f29","f30","f31",
            "", "pc","ps","cr","lr","ctr","xer"]"#;

        println!("{:#?}", parse(str));
    }
}
