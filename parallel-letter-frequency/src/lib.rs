use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::cmp;


pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Init concurrent vec
    let concurrent_vec = Arc::new(Mutex::new(vec![]));
    let mut children = vec![];

    // Calculate each worker load
    let str_len = input.len();
    let running_worker = cmp::min(str_len, worker_count);
    let each_worker_load = str_len.div_euclid(running_worker);

    // Convert char to map
    let char_to_map = |c: char| {
        let mut h = HashMap::new();
        h.insert(c.to_ascii_lowercase(), 1);
        h
    };

    // Join many maps into one
    let fold_map = |a: HashMap<char, usize>, b: HashMap<char, usize>| {
        let zero: usize = 0;
        let mut c: HashMap<char, usize> = a.into_iter().collect();
        for (k, v) in b.iter() {
            c.insert(*k, c.get(k).unwrap_or(&zero) + v);
        }
        c
    };


    for i in 0..running_worker {
        let clone_v = concurrent_vec.clone();
        let string = input[i * each_worker_load..cmp::min((i + 1) * each_worker_load, str_len)]
            .join("");

        children.push(thread::spawn(move || {
            let h = string.chars()
                .filter(|c| c.is_alphabetic())
                .map(char_to_map)
                .fold(HashMap::new(), fold_map);
            clone_v.lock().unwrap().push(h);
        }));
    };

    // Wait for all workers finish
    for child in children {
        let _ = child.join();
    };

    // Calculate final result
    let cv = concurrent_vec.lock().unwrap().clone();
    cv.into_iter().fold(HashMap::new(), fold_map)
}
