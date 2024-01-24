fn take_third(value: Vec<i32>) -> Option<i32> {
    if value.len() < 3 {
        None
    } else {
        Some(value[2])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_num = take_third(vec);
        if inside_num.is_some() {
            println!("We got: {}", inside_num.unwrap());
        } else {
            println!("We got nothing");
        }
    }
}
