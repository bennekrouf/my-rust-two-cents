pub fn main() {
    let v = vec![3,6,7,8,6,2];
    let sublist: Vec<i32> = v.into_iter()
                             .filter(|x| *x % 2 == 0)
                             .collect();
                             
    println!("{:?}", sublist);
}