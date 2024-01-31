fn main() {
    let first_num = 10;
    let second_num = 6;

    let my_closure = |x: i32| println!("{}", first_num + second_num + x);
    my_closure(4);
}
