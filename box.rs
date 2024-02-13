// with Box, you can put a type on the heap instead of stack
fn main() {
    let my_box = Box::new(1);
    let an_integer = *my_box;
    println!("{:?}", my_box);
    println!("{:?}", an_integer);
}
