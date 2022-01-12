use std::time::PrimitiveDateTime as DateTime;

pub fn _reverse(input: &str) -> String {
    let a: Vec<char> = input.chars().collect();
    a.into_iter().rev().collect()
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    println!("{:?}", start);
    start
}


pub fn main() {
    let a: DateTime = DateTime::new();
    after(a);
}