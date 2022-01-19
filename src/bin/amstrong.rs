pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let mut temp = 0;
    for (_i, val) in s.chars().enumerate() {
        temp += (val as u32 - 48).pow(s.len() as u32);
    }
    temp == num
}

pub fn main() {
    let a = 9474;
    println!("{} is an amstrong number ? {:?}", a, is_armstrong_number(a));
}

