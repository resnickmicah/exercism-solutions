#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() > _second_list.len() {
        if _second_list.len() == 0 {
            return Comparison::Superlist;
        }
        let first_sublists = generate_sublists(_second_list.len(), _first_list);
        for sublist in first_sublists {
            if compare_slices(sublist, _second_list) == Comparison::Equal {
                return Comparison::Superlist;
            }
        }
        return Comparison::Unequal;
    }

    if _first_list.len() < _second_list.len() {
        if _first_list.len() == 0 {
            return Comparison::Sublist;
        }
        let second_sublists = generate_sublists(_first_list.len(), _second_list);
        for sublist in second_sublists {
            if compare_slices(sublist, _first_list) == Comparison::Equal {
                return Comparison::Sublist;
            }
        }
        return Comparison::Unequal;
    }
    compare_slices(_first_list, _second_list)
}

pub fn generate_sublists<T: PartialEq>(sublist_length: usize, source_list: &[T]) -> Vec<&[T]> {
    assert!(sublist_length < source_list.len());
    assert!(sublist_length > 0);
    let mut retval: Vec<&[T]> = Vec::new();
    for i in 0..(source_list.len() - sublist_length + 1) {
        retval.push(&source_list[i..i + sublist_length]);
    }
    retval
}

pub fn compare_slices<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    assert!(first.len() == second.len());
    for i in 0..first.len() {
        if first[i] != second[i] {
            return Comparison::Unequal;
        }
    }
    Comparison::Equal
}
