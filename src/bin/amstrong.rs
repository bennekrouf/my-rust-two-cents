pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let mut b = s.chars();
    let mut result = 0;
    for i in &mut b {
        result += i as u32 - 48 ^ b.count() as u32;
    }
    num == result
}

pub fn main() {
    let a = 153;
    println!("{:?}", is_armstrong_number(a));
}

