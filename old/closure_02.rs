fn main() {
    let num_vec = vec![2, 4, 6];

    let double_vec = num_vec
        .iter()
        .map(|number| number * 2)
        .collect::<Vec<i32>>();
    println!("{:?}", double_vec);

    num_vec
        .iter()
        .enumerate()
        .for_each(|(index, number)| println!("Index number {} has number {}", index, number));
}
