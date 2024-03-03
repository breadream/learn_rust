fn main() {
    let mut my_vec = vec![100, 90, 80, 0, 0, 0, 0];
    let mut copied_vec = my_vec.clone();
    my_vec.sort(); // relative order is guaranteed
    copied_vec.sort_unstable();
    println!("{:?}", my_vec);
    println!("{:?}", copied_vec);
    copied_vec.dedup();
    println!("{:?}", copied_vec);
}
