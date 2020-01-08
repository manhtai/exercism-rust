use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::cmp;


pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // For default value
    let zero: usize = 0;

    // Init concurrent vec
    let concurrent_vec = Arc::new(Mutex::new(vec![]));
    let mut children = vec![];

    // Calculate each worker load
    let str_len = input.len();
    let running_worker = cmp::min(str_len, worker_count);
    let each_worker_load = str_len.div_euclid(running_worker);

    for i in 0..running_worker {
        let clone_v = concurrent_vec.clone();
        let string = input[i * each_worker_load..cmp::min((i + 1) * each_worker_load, str_len)]
            .join("");

        children.push(thread::spawn(move || {
            let mut h: HashMap<char, usize> = HashMap::new();
            for c in string.chars() {
                if !c.is_alphabetic() {
                    continue;
                }

                let lc = c.to_ascii_lowercase();
                h.insert(lc, h.get(&lc).unwrap_or(&zero) + 1);
            }
            clone_v.lock().unwrap().push(h);
        }));
    };

    // Wait for all workers finish
    for child in children {
        let _ = child.join();
    };

    // Calculate final result
    let cv = concurrent_vec.lock().unwrap().clone();
    cv
        .into_iter()
        .fold(HashMap::new(), |a, b| {
            let mut c: HashMap<char, usize> = a.into_iter().collect();
            for (k, v) in b.iter() {
                c.insert(*k, c.get(k).unwrap_or(&zero) + v);
            }
            c
        })
}
