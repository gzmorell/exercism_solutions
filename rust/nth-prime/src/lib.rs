pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|&number| {
            if primes.iter().filter(|&&prime| prime < (number / 2 + 1)).all(|prime| number % prime != 0) {
                primes.push(number);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
