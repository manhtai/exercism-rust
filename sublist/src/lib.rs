#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let (larger_list, smaller_list, cmp) = if _first_list.len() > _second_list.len() {
        (_first_list, _second_list, 1)
    } else if _first_list.len() == _second_list.len() {
        (_first_list, _second_list, 0)
    } else {
        (_second_list, _first_list, -1)
    };

    let mut is_sub = true;
    'outer: for i in 0..(larger_list.len() - smaller_list.len() + 1) {
        'inner: for j in 0..smaller_list.len() {
            if larger_list[i+j] != smaller_list[j] {
                is_sub = false;
                break 'inner;
            }
        }

        if is_sub {
            break 'outer;
        }
    }

    if !is_sub {
        Comparison::Unequal
    } else {
        match cmp {
            1 => Comparison::Superlist,
            -1 => Comparison::Sublist,
            _ => Comparison::Equal,
        }
    }
}
