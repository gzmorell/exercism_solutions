use std::collections::{HashMap, HashSet};

fn histogram(word: &str) -> HashMap<char, usize> {
    let mut hist: HashMap<char, usize> = HashMap::new();
    for c in word.chars() {
        let count = hist.entry(c).or_insert(0);
        *count += 1;
    }
    hist
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    // unimplemented!(
    //     "For the '{}' word find anagrams among the following words: {:?}",
    //     word,
    //     possible_anagrams
    // );

    let lowercase_word = word.to_lowercase();
    let lowercase_word_histogram = histogram(&lowercase_word);
    let lowercase_word_size = lowercase_word.len();

    possible_anagrams
        .iter()
        .inspect(|&anagram| println!("{}", anagram))
        .filter(|&anagram| {
            let lowercase_anagram = anagram.to_lowercase();
            lowercase_anagram.len() == lowercase_word_size
                && lowercase_anagram != lowercase_word
                && {
                    let mut lowercase_word_histogram_copy = lowercase_word_histogram.clone();
                    lowercase_anagram.chars().all(|c| {
                        if let Some(x) = lowercase_word_histogram_copy.get_mut(&c) {
                            *x -= 1;
                            if *x == 0 {
                                lowercase_word_histogram_copy.remove_entry(&c);
                            }
                            true
                        } else {
                            false
                        }
                    }) && lowercase_word_histogram_copy.is_empty()
                }
        })
        .inspect(|&anagram| println!("{}", anagram))
        .copied()
        .collect()
}

#[test]
fn test_empty_string_histogram() {
    let word_histogram: HashMap<char, usize> = HashMap::new();
    let result = histogram("");
    assert_eq!(word_histogram, result);
}

#[test]
fn test_word_histogram() {
    let mut word_histogram: HashMap<char, usize> = HashMap::new();
    word_histogram.insert('h', 1);
    word_histogram.insert('e', 1);
    word_histogram.insert('l', 2);
    word_histogram.insert('o', 1);
    let result = histogram("hello");
    assert_eq!(word_histogram, result);
}
