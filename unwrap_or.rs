fn main() {
    let my_vec = vec![8, 9, 10];
    
    let fourth = my_vec.get(3).unwrap_or(&0);

    println!("{}", fourth);
}
