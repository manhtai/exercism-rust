use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::borrow::Borrow;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let concurrent_vec = Arc::new(Mutex::new(vec![]));
    let mut children = vec![];

    for i in 0..input.len() {
        let clone_v = concurrent_vec.clone();
        let string = input[i].to_owned();
        children.push(thread::spawn(move || {
            let mut h: HashMap<char, usize> = HashMap::new();
            for c in string.chars() {
                if !c.is_alphabetic() {
                    continue;
                }

                let lc = c.to_ascii_lowercase();

                if h.get(&lc).is_none() {
                    h.insert(lc, 1);
                } else {
                    h.insert(lc, h.get(&lc).unwrap() + 1);
                }
            }
            clone_v.lock().unwrap().push(h);
        }));
    };

    for child in children {
        let _ = child.join();
    };

    let cv = concurrent_vec.lock().unwrap().clone();
    let zero: usize = 0;

    cv.into_iter()
        .fold(HashMap::new(), |a, b| {
            let mut c: HashMap<char, usize> = a.into_iter().collect();
            for (k, v) in b.iter() {
                c.insert(*k, c.get(k).unwrap_or(&zero) + v);
            }
            c
        })
}
