use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut max_hand = Hand { order: 0, ranks: vec![0] };
    let mut result = vec![];

    for &hand_str in hands {
        let hand = get_hand_rank(hand_str);

        if result.is_empty() || hand > max_hand {
            result = vec![hand_str];
            max_hand = hand;
        } else if max_hand == hand {
            result.push(hand_str);
        }
    }

    return Some(result);
}

#[derive(Eq)]
struct Hand {
    order: i32,
    ranks: Vec<usize>,
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.order.cmp(&other.order) {
            Ordering::Equal => self.ranks.cmp(&other.ranks),
            neq => neq,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other).eq(&Ordering::Equal)
    }
}

fn to_number(h: &str) -> usize {
    match h {
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        _ => h.parse::<usize>().unwrap(),
    }
}

fn find_hand(h: &str) -> (&str, usize) {
    if h.len() == 2 {
        (&h[1..2], to_number(&h[0..1]))
    } else {
        (&h[2..3], to_number(&h[0..2]))
    }
}

fn get_hand_rank(hand: &str) -> Hand {
    let mut groups = HashMap::new();
    let mut keys = HashSet::new();

    for s in hand.split_whitespace() {
        let (key, rank) = find_hand(s);
        (*groups.entry(rank).or_insert(0)) += 1;
        keys.insert(key);
    }

    let mut counts = vec![];
    let mut ranks = vec![];
    let mut cmp_ranks = vec![];

    let mut groups = groups.into_iter().collect::<Vec<(usize, i32)>>();
    groups.sort_by(|(r1, c1), (r2, c2)| {
        match c1.cmp(c2) {
            Ordering::Equal => r2.cmp(r1),
            neq => neq.reverse(),
        }
    });

    for (rank, count) in groups {
        ranks.push(rank);
        cmp_ranks.push(rank);
        counts.push(count);
    }
    cmp_ranks.sort_by_key(|a| Reverse(*a));

    if cmp_ranks.eq(&vec![14, 5, 4, 3, 2]) {
        ranks = vec![5, 4, 3, 2, 1]
    };

    let longest_path = ranks.iter().max().unwrap() - ranks.iter().min().unwrap();
    let straight = ranks.len() == 5 && longest_path == 4;
    let flush = keys.len() == 1;

    let order = if counts.eq(&vec![5]) {
        9
    } else if straight & flush {
        8
    } else if counts.eq(&vec![4, 1]) {
        7
    } else if counts.eq(&vec![3, 2]) {
        6
    } else if flush {
        5
    } else if straight {
        4
    } else if counts.eq(&vec![3, 1, 1]) {
        3
    } else if counts.eq(&vec![2, 2, 1]) {
        2
    } else if counts.eq(&vec![2, 1, 1, 1]) {
        1
    } else {
        0
    };

    Hand { order, ranks }
}
