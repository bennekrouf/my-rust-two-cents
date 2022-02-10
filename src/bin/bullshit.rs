use core::fmt::Debug;

impl Debug for dyn ToString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Series{{{}}}", self.len())
    }
}


pub fn main(){
    let c:[i32; 3] = [0; 3];
    let b:Vec<i32> = vec![0; 3];
    let a:Vec<Box<dyn ToString>> = vec![Box::new("str"), Box::new(String::from("string"))];

    println!("Array {:?}", c);
    println!("Vec {:?}", b);
    println!("Mix types {}", a);
}