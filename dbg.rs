fn main() {
    let mut my_num = dbg!(9);
    dbg!(my_num += 10);
    
    let new_vec = dbg!(vec![8, 9, 10]);

    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());

    dbg!(double_vec);
}
