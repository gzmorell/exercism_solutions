pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
    (1..limit)
        .filter(|number| {
            factors
                .iter()
                .any(|factor| {
                    if *factor != 0 {
                        number % factor == 0
                    } else {
                        false
                    }
                })
        })
        .sum()
}
