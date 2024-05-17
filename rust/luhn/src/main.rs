/// Check a Luhn checksum.
fn is_string_only_ints(s: &str) -> bool{
    s.chars().all(|x| x.is_digit(10))
}

fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    // reverse the string
    let no_whitespace: String = code.split_whitespace().collect();
    println!("{}", no_whitespace);
    if !is_string_only_ints(&no_whitespace) {
        return false;
    }

    // Collect them into a vector of u32s
    let mut rev_vec: Vec<u32> = no_whitespace.chars().rev().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();

    //Double every second digit
    for (idx, int) in rev_vec.iter_mut().enumerate() {
        if idx % 2 == 1 {
            if *int == 9 {
                *int = 9;
            } else {
                *int = (*int * 2) % 9;
            }
        }
    }
    
    //Sum the digits in the rev_vec
    let sum: u32 = rev_vec.iter().fold(0, |acc, x| acc + x);
    if sum % 10 == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("{}", is_valid("234 567 891 234"));
    println!("{}", is_valid("095 245 88"));
    println!("{}", is_valid(" 0"));
 
}
