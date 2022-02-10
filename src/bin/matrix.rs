fn biggest_row(a: &Vec<Vec<i32>>) -> usize {
    let mut res:usize = 0;
    let mut biggest_sum = a[0].iter().sum::<i32>();
    let mut new_sum;
    for (index, val) in a.iter().enumerate() {
        new_sum = val.iter().sum::<i32>();
        if new_sum > biggest_sum {
            biggest_sum = new_sum;
            res = index;
        }
    }
    res
}

fn column(a: &Vec<Vec<i32>>, index: usize) -> Vec<i32> {
    let mut res:Vec<i32> = vec![];
    for (_, val) in a.iter().enumerate() {
        res.push(val[index]);
    }
    res
}

fn biggest_column(a: &Vec<Vec<i32>>) -> usize {
    let mut inversed_matrix: Vec<Vec<i32>> = vec![];
    for (i, _) in a.iter().enumerate() {
        inversed_matrix.push(column(a, i));
    }
    biggest_row(&inversed_matrix)
}

fn flip_column(a: &Vec<Vec<i32>>, from: usize, to: usize) -> Vec<Vec<i32>> {
    let mut res:Vec<Vec<i32>> = vec![];
    for (_, row) in a.iter().enumerate() {
        let mut new_row: Vec<i32> = vec![];
        for (_, item) in row.iter().enumerate() {
            new_row.push(*item);
        }
        new_row[from] = row[to];
        new_row[to] = row[from];
        res.push(new_row);
    }
    res
}

pub fn main(){
    let mut a:Vec<Vec<i32>> = vec![
        vec![112,245,83,119],
        vec![56,125,56,49],
        vec![15,78,101,443],
        vec![62,98,114,108]
        ];
    for (_, val) in a.iter().enumerate() {
        println!("{:?}", val);
    }

    println!(" ----- ");

    // println!("Biggest row :{:?}", biggest_row(&a));
    // println!("Biggest column :{:?}", biggest_column(&a));
    let f: usize = biggest_column(&a);
    if f != 0 {
        a = flip_column(&a, 0, f);
    }
    for (_, val) in flip_column(&a, 1, 3).iter().enumerate() {
        println!("{:?}", val);
    }
}