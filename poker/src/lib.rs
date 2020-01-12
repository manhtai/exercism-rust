use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    find_all_winning_hands(hands)
}

#[derive(Eq)]
struct Hand {
    rank: usize,
    key: String,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

fn find_all_winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut max_val = 0;
    let mut result = vec![];

    for &hand in hands {
        let hand_rank = get_hand_rank(hand);
        if result.is_empty() || hand_rank > max_val {
            result = vec![hand];
        } else if hand_rank == max_val {
            result.push(hand);
        }
    }

    return Some(result);
}


fn get_hand_rank(hand: &str) -> i32 {
    let mut groups = HashMap::new();
    let hands = hand
        .to_string()
        .split_whitespace()
        .map(|s| {
            let key = &s[0..1];
            let rank = "--23456789TJQKA".find(key).unwrap();
            (*groups.entry(rank).or_insert(0)) += 1;
            Hand { rank, key: key.to_owned() }
        })
        .collect::<Vec<Hand>>();

    let mut hands_vec = groups
        .iter()
        .collect::<Vec<(usize, usize)>>();
    hands_vec.sort();

    let counts = hands_vec.iter().map(|h| h.rank).collect::<Vec<usize>>();
    let mut ranks = hands_vec.iter().map(|h| h.count).collect::<Vec<usize>>();

    if ranks == vec![14, 5, 4, 3, 2] {
        ranks = vec![5, 4, 3, 2, 1];
    }
    let straight = ranks.len() == 5 && (ranks.max() - ranks.min()) == 4;
    let flush = hands.iter().map(|h| h.key).collect::<HashSet<String>>().len() == 1;

    if counts == vec![5] {
        9
    } else if straight & flush {
        8
    } else if counts == vec![4, 1] {
        7
    } else if counts == vec![[3, 2]] {
        6
    } else if flush {
        5
    } else if straight {
        4
    } else if counts == vec![3, 1, 1] {
        3
    } else if counts == vec![2, 2, 1] {
        2
    } else if counts == vec![2, 1, 1, 1] {
        1
    } else {
        0
    }
}
