#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() != _second_list.len() {
        if _first_list.len() > _second_list.len() {
            // superlist
            if _second_list.is_empty() || _first_list.windows(_second_list.len()).any(|window| window == _second_list) {
                return Comparison::Superlist;
            }
        } else {
            // sublist
            if _first_list.is_empty() || _second_list.windows(_first_list.len()).any(|window| window == _first_list) {
                return Comparison::Sublist;
            }
        }
    } else {
        // equal
        if _first_list.iter().zip(_second_list.iter()).all(|(elem1, elem2)| elem1 == elem2) {
            return Comparison::Equal;
        }
    }
    Comparison::Unequal
}
