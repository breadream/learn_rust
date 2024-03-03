use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_display<T, U> (stmt: T, num_1: U, num_2: U) 
where
    T: Display,
    U: Display + PartialOrd,
{
    println!("{}! Is {} greater than {} ? {}", stmt, num_1, num_2, num_1 > num_2);
}

fn main() {
    compare_and_display("Hey this is", "123", "321");
}
