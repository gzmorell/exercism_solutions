use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _first_list.is_empty() {
        true
    } else {
        match _second_list
            .iter()
            .position(|value| value == &_first_list[0])
        {
            Some(index) => {
                if _first_list.len() <= _second_list.len() - index {
                    if _first_list
                        .iter()
                        .eq(_second_list[index..index + _first_list.len()].iter())
                    {
                        true
                    } else {
                        is_sublist(_first_list, &_second_list[index + 1..])
                    }
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

fn is_superlist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    is_sublist(_second_list, _first_list)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Less => match is_sublist(_first_list, _second_list) {
            true => Comparison::Sublist,
            false => Comparison::Unequal,
        },
        Ordering::Equal => match _first_list.iter().eq(_second_list.iter()) {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },
        Ordering::Greater => match is_superlist(_first_list, _second_list) {
            true => Comparison::Superlist,
            false => Comparison::Unequal,
        },
    }
}
