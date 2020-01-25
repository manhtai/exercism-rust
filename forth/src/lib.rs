use crate::Error::{UnknownWord, InvalidWord};

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: vec![] }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.to_vec()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let result = input.split_whitespace().map(|ops| {
            if ops.chars().all(|c| c.is_numeric()) {
                let val = ops.parse::<Value>();
                match val {
                    Ok(v) => {
                        self.stack.push(v);
                        Ok(())
                    }
                    _ => Err(InvalidWord),
                }
            } else {
                match input {
                    "+" | "-" | "*" | "/" | "SWAP" => {
                        let a_opt = self.stack.pop();
                        let b_opt = self.stack.pop();
                        if a_opt.is_some() && b_opt.is_some() {
                            let a = a_opt.unwrap();
                            let b = b_opt.unwrap();

                            match input {
                                "+" => self.stack.push(a + b),
                                "-" => self.stack.push(a - b),
                                "*" => self.stack.push(a * b),
                                "/" => self.stack.push(a / b),
                                _ => {
                                    self.stack.push(b);
                                    self.stack.push(a);
                                }
                            };
                            Ok(())
                        } else {
                            Err(InvalidWord)
                        }
                    }
                    "DUP" => {
                        let val = self.stack.pop();
                        match val {
                            Some(v) => {
                                self.stack.push(v * 2);
                                Ok(())
                            }
                            _ => Err(InvalidWord),
                        }
                    }
                    "DROP" => {
                        self.stack.pop();
                        Ok(())
                    }
                    "OVER" => {
                        if self.stack.len() < 2 {
                            return Err(InvalidWord);
                        }
                        let one_to_last = self.stack[self.stack.len() - 2];
                        self.stack.push(one_to_last);
                        Ok(())
                    }
                    _ => Err(UnknownWord),
                }
            }
        });

        match result.filter(|r| r.is_err()).next() {
            Some(e) => e,
            _ => Ok(())
        }
    }
}
