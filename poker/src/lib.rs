use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    find_all_winning_hands(hands)
}

fn find_all_winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let (mut max_point, mut max_rank) = (0, vec![]);
    let mut result = vec![];

    for &hand in hands {
        let (point, rank) = get_hand_rank(hand);
        let lt = if point > max_point {
            true
        } else if point == max_point {
            rank.gt(&max_rank)
        } else {
            false
        };

        let eq = point == max_point && rank.eq(&max_rank);

        if result.is_empty() || lt {
            result = vec![hand];
            max_point = point;
            max_rank = rank;
        } else if eq {
            result.push(hand);
        }
    }

    return Some(result);
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

fn get_hand_rank(hand: &str) -> (i32, Vec<usize>) {
    let mut groups = HashMap::new();
    let keys = hand
        .split_whitespace()
        .map(|s| {
            let (key, rank) = find_hand(s);
            (*groups.entry(rank).or_insert(0)) += 1;
            key
        })
        .collect::<HashSet<&str>>();

    let mut hands_vec = groups.iter().collect::<Vec<(&usize, &i32)>>();
    hands_vec.sort();

    let counts = hands_vec
        .iter()
        .map(|(_, count)| **count)
        .collect::<Vec<i32>>();

    let mut ranks = hands_vec
        .iter()
        .map(|(rank, _)| **rank)
        .collect::<Vec<usize>>();

    ranks = if ranks.eq(&vec![14, 5, 4, 3, 2]) {
        vec![5, 4, 3, 2, 1]
    } else {
        ranks
    };

    let straight =
        ranks.len() == 5 && (ranks.iter().max().unwrap() - ranks.iter().min().unwrap()) == 4;
    let flush = keys.len() == 1;

    let sort = if counts.eq(&vec![5]) {
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

    (sort, ranks)
}
