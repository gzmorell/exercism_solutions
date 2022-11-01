/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

struct Fib {
    first: Option<u8>,
    second: Option<u8>,
}

impl Fib {
    fn new() -> Fib {
        Fib {
            first: Some(1),
            second: Some(1),
        }
    }
}

impl Iterator for Fib {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.first;
        if let Some(x) = result {
            self.first = self.second;
            if let Some(y) = self.second {
                if (x as usize + y as usize) > Self::Item::MAX as usize {
                    self.second = None;
                } else {
                    self.second = Some(x + y);
                }
            }
        }
        result
    }
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    Fib::new().take(5).collect()
}
