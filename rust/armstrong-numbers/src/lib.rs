pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    let num_string = num.to_string();
    let power = num_string.len() as u32;
    num_string
        .chars()
        .map(|x| (x.to_digit(10).unwrap()).pow(power))
        .sum::<u32>()
        == num
}
