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

    let is_sub = is_sublist(smaller_list, larger_list);

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

fn is_sublist<T: PartialEq>(smaller_list: &[T], larger_list: &[T]) -> bool {
    if larger_list.len() == 0 || smaller_list.len() == 0 {
        return true;
    }

    for window in larger_list.windows(smaller_list.len()) {
        if window == smaller_list {
            return true;
        }
    }

    return false;
}
