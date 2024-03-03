use std::fmt::{Debug, Display};

fn print_it<T>(input: T)
where
    T: AsRef<str> + Debug + Display,
{
    println!("{}", input);
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}
