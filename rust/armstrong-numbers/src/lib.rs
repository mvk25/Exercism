pub fn is_armstrong_number(num: u32) -> bool {
    // Convert the numbers to String, split them and act on them individually
    let num_as_string = num.to_string();
    let num_vec = num_as_string.chars().map(|x| x.to_digit(10).unwrap_or(0) as u64).collect::<Vec<u64>>();
    
    let num_digit = num_vec.len() as u64;
    let mut sum: u64 = 0;

    for i in num_vec {
        sum += i.pow(num_digit as u32);
    }

    sum == num as u64
}
