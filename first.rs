pub struct Foo {
    value: u8
}

fn main() {
    let foo = Foo { value: 42 };
    let bar = foo;
    println!("Here is ... {}!", bar.value);
}