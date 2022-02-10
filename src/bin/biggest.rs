pub fn bigme(n: Vec<i32>) -> i32 {
    let mut biggest = n[0];
    for i in n {
        if i > biggest {
            biggest = i;
        }
    }
    biggest
}

pub fn main() {
    let t:Vec<i32> = vec![54,544,76,32,6,5];
    let biggest = bigme(t);
    println!("{}", biggest);
}