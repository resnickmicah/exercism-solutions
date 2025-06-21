// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words: HashMap<&str, u16> = HashMap::new();
    let mut note_words: HashMap<&str, u16> = HashMap::new();
    println!("Magazine input: {:?}", magazine);
    println!("Note input: {:?}", note);

    for word in magazine {
        *magazine_words.entry(word).or_default() += 1;
    }

    for word in note {
        *note_words.entry(word).or_default() += 1;
    }

    println!("Magazine word counts: {:?}", magazine_words);
    println!("Note word counts: {:?}", note_words);

    for (word, note_count) in note_words {
        let magazine_count = if magazine_words.contains_key(word) {
            magazine_words[word]
        } else {
            0
        };
        if magazine_count < note_count {
            println!(
                "Not enough {:?} in magazine: have {:?}, need {:?}",
                word, magazine_count, note_count
            );
            return false;
        }
    }
    return true;
}
