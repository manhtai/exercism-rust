use crate::Error::{UnknownWord, InvalidWord, StackUnderflow, DivisionByZero};
use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    env: HashMap<String, Vec<String>>,
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
        Forth { stack: vec![], env: HashMap::new() }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.to_vec()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let input_vec: Vec<String> = input.split_whitespace().map(|s| s.to_lowercase()).collect();

        // Define words
        if input_vec.starts_with(&[":".to_owned()]) {
            if input_vec.len() < 4 {
                return Err(InvalidWord);
            }

            let word = input_vec[1].to_owned();
            let definition = input_vec[2..input_vec.len() - 1].to_vec();
            if word.chars().any(|c| c.is_numeric()) {
                return Err(InvalidWord);
            }

            self.env.insert(word, self.extend_def(definition));
            return Ok(());
        }

        let extended_vec = self.extend_def(input_vec);

        // Operator
        let result = extended_vec.into_iter().map(|ops| {
            if ops.chars().all(|c| c.is_numeric()) {
                let val = ops.parse::<Value>();
                return match val {
                    Ok(v) => {
                        self.stack.push(v);
                        Ok(())
                    }
                    _ => Err(InvalidWord),
                };
            }

            match ops.as_str() {
                "+" | "-" | "*" | "/" | "swap" => {
                    let a_opt = self.stack.pop();
                    let b_opt = self.stack.pop();
                    if a_opt.is_none() || b_opt.is_none() {
                        return Err(StackUnderflow);
                    }

                    let a = a_opt.unwrap();
                    let b = b_opt.unwrap();

                    match ops.as_str() {
                        "+" => self.stack.push(a + b),
                        "-" => self.stack.push(b - a),
                        "*" => self.stack.push(a * b),
                        "/" => {
                            if a == 0 {
                                return Err(DivisionByZero);
                            }
                            self.stack.push(b / a);
                        }
                        // swap
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

    fn extend_def(&self, input_vec: Vec<String>) -> Vec<String> {
        // Extend words
        let mut extended_vec: Vec<String> = Vec::new();
        for v in input_vec {
            let key = v.as_str();
            if self.env.contains_key(key) {
                extended_vec.extend_from_slice(self.env.get(key).unwrap());
            } else {
                extended_vec.push(key.to_string());
            }
        }
        extended_vec
    }
}
