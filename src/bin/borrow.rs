pub fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("{}", boxed_i32);
}

pub fn borrow_i32(borrowed_i32: &i32) {
    println!("{}", borrowed_i32);
}

pub fn main() {
    let boxed_i32 = Box::new(500_i32);
    let stacked_i32 = 6_i32;

    // borrow_i32(boxed_i32); expected `&i32`, found struct `Box` consider borrowing
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // eat_box_i32(boxed_i32);
        

        // Attempt to borrow ref_to_i32 after inner value is destroyed
        borrow_i32(_ref_to_i32);

    }

    eat_box_i32(boxed_i32); // value used here after move if line 21 active
}