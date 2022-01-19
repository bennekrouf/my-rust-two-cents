pub fn main() {
    let a: String = String::from("hello world!");
    let c: &str = first_word(&a);
    println!("{}", c);

}

pub fn first_word(b: &str) -> &str {
    let bytes: &[u8] = b.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &b[0..i];
        }
    }
    &b[..]
}