pub fn main() {
    let a: &str = "toto";
    let _c = a.to_string().pull();
    // let c = a.chars().filter(|c| *c == 'o').map(|c| c).collect();
    let _b: String = "toto".to_owned();
    // c.next();
    println!("{:?}", _c);
}