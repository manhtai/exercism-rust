use std::cmp;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Init concurrent vec
    let mut children = vec![];

    // Calculate each worker load
    let str_len = input.len();
    if str_len == 0 {
        return HashMap::new();
    }

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

    // Spawn workers to do work
    for i in 0..running_worker {
        let string =
            input[i * each_worker_load..cmp::min((i + 1) * each_worker_load, str_len)].join("");

        children.push(thread::spawn(move || {
            string
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(char_to_map)
                .fold(HashMap::new(), fold_map)
        }));
    }

    // Wait for all workers finish
    children
        .into_iter()
        .map(|c| c.join().unwrap())
        .fold(HashMap::new(), fold_map)
}
