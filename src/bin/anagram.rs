use std::collections::HashSet;
use std::*;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut res:HashSet<&'a str> = HashSet::new();
    let mut the_word: Vec<char> = word.to_lowercase().chars().collect();
    the_word.sort();
    let the_word: String = the_word.iter().collect();
    for item in possible_anagrams {
        let mut sorted_possible: Vec<char> = item.to_lowercase().chars().collect();
        sorted_possible.sort();
        let sorted_possible: String = sorted_possible.iter().collect();

        match sorted_possible {
            the_word => {
                res.insert(&item);
            },
            _ => println!("toto"),
        }
    }
    res
}

pub fn main() {
    let a = ["enlists", "google", "inlets", "banana"];
    println!("{:?}", anagrams_for("listen", &a));
}
