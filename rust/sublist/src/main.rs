// use core::num;

enum Comparison {
    Equal,
    NotEqual,
}

fn compare_vectors<T: PartialEq>(vec1: &[T], vec2: &[T]) -> Comparison {
    if vec1.len() != vec2.len() {
        return Comparison::NotEqual;
    }

    for (elem1, elem2) in vec1.iter().zip(vec2.iter()) {
        if elem1 != elem2 {
            return Comparison::NotEqual;
        }

    }
    Comparison::Equal
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    let vec3 = vec![1, 2, 4];

    match compare_vectors(&vec1, &vec2) {
        Comparison::Equal => println!("vec1 and vec2 are equal"),
        Comparison::NotEqual => println!("vec1 and vec2 are not equal"),
    }

    match compare_vectors(&vec1, &vec3) {
        Comparison::Equal => println!("vec1 and vec2 are equal"),
        Comparison::NotEqual => println!("vec1 and vec3 are not equal"),
    }

    let slice = ['w', 'i', 'n', 'd', 'o', 'w', 's'];
    for window in slice.windows(2) {
        println!("[{} {}]", window[0], window[1])
    }
}