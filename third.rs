fn get_even_numbers(v: Vec<i32>) -> Option<Vec<i32>> {
    let mut res: Vec<i32> = Vec::new();
    for i in v.into_iter() {
        if i % 2 == 0 {
            res.push(i.clone())
        }
    }

    if res.len() > 0 {
        Some(res)
    } else {
        None
    }
}

fn main() {
    let vec_option1 = get_even_numbers(vec![1,2,3,4]);
    println!("{:?}", vec_option1.unwrap());

    let vec_option2 = get_even_numbers(vec![1,5,6,7]);
    println!("{:?}", vec_option2.unwrap());
}