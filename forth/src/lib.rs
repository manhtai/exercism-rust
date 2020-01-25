use crate::Error::{UnknownWord, InvalidWord, StackUnderflow, DivisionByZero};

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
                return match val {
                    Ok(v) => {
                        self.stack.push(v);
                        Ok(())
                    }
                    _ => Err(InvalidWord),
                }
            }

            match ops.to_lowercase().as_str() {
                "+" | "-" | "*" | "/" | "swap" => {
                    let a_opt = self.stack.pop();
                    let b_opt = self.stack.pop();
                    if a_opt.is_none() || b_opt.is_none() {
                        return Err(StackUnderflow);
                    }

                    let a = a_opt.unwrap();
                    let b = b_opt.unwrap();

                    match ops {
                        "+" => self.stack.push(a + b),
                        "-" => self.stack.push(b - a),
                        "*" => self.stack.push(a * b),
                        "/" => {
                            if a == 0 {
                                return Err(DivisionByZero);
                            }
                            self.stack.push(b / a);
                        }
                        _ => {
                            self.stack.push(a);
                            self.stack.push(b);
                        }
                    };
                    Ok(())
                }
                "dup" => {
                    if self.stack.len() < 1 {
                        return Err(StackUnderflow);
                    }
                    let val = self.stack[self.stack.len() - 1];
                    self.stack.push(val);
                    Ok(())
                }
                "drop" => {
                    if self.stack.len() < 1 {
                        return Err(StackUnderflow);
                    }
                    self.stack.pop();
                    Ok(())
                }
                "over" => {
                    if self.stack.len() < 2 {
                        return Err(StackUnderflow);
                    }
                    let one_to_last = self.stack[self.stack.len() - 2];
                    self.stack.push(one_to_last);
                    Ok(())
                }
                _ => Err(UnknownWord),
            }
        });

        match result.filter(|r| r.is_err()).next() {
            Some(e) => e,
            _ => Ok(())
        }
    }
}
