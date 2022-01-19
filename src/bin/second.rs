#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Employee {
    name: String
}

impl Into<Employee> for Person {
    fn into(self) -> Employee {
        Employee {
            name: self.name
        }
    }
}

fn main() {
    let person: Person = Person { name: "toto".to_string()};
    let employee: Employee = person.into();

    println!("{}", employee.name)
}