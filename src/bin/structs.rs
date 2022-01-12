#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    fn new(name: String, age: u32, weight: f32) -> User {
        User { name: name, age: age, weight: weight}
    }

    fn weight(&self) -> f32{
        self.weight
    }

    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

pub fn main() {
    let mut u: User = User::new(String::from("Bob"), 32, 155.2);
    println!("{} {:?} {:?}", u.name, u.age, u.weight());

    u.set_age(5);
    println!("{}", u.age);
}