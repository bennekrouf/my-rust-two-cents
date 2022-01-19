use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut res:HashSet<&'a str> = HashSet::new();
    for item in possible_anagrams {
        for (_, val) in word.chars().enumerate() {
            
        }

        // if !word.eq(*item) {
        //     let mut is = 0;
        //     for (_i, val) in word.chars().enumerate() {
        //         if item.contains(val) {
        //             is += 1
        //         }
        //     }
        //     if is == word.len() {
        //         res.insert(item);
        //     }
        // }
    }
    res
}

pub fn main() {
    let a = ["enlists", "google", "inlets", "banana"];
    println!("{:?}", anagrams_for("listen", &a));
}
