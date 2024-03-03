fn main() {
    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|first_item| {
            println!("The item is: {}", first_item);
            match **first_item % 2 {
                0 => println!("It's even"),
                _ => println!("It's odd"),
            }
            println!("in binary it is {:b}", first_item);
        })
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
}
