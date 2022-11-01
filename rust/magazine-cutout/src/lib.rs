// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut notes = note.iter().fold(HashMap::new(), |mut words, str| {
        *words.entry(str).and_modify(|e| *e += 1).or_insert(1);
        words
    });
    magazine.iter().for_each(|str| {
        notes
            .entry(str)
            .and_modify(|e| *e = if *e == 0 { 0 } else { *e - 1 });
    });
    notes.values().sum::<i32>() == 0
}
