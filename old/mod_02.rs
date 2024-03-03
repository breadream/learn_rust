mod print_things {
    use std::fmt::{Display, Debug};

    #[derive(Debug)]
    pub struct Billy { // Billy is public
        name: String, // but name is private
        pub times_to_print: u32,
    }

    impl Billy {
        pub fn new(times_to_print: u32) -> Self {
            Self {
                name: "Billy".to_string(),
                times_to_print,
            }
        }

        pub fn print_billy(&self) {
            for _ in 0..self.times_to_print {
                println!("{:?}", self.name);
            }
        }
    }

    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input);
    }
}

fn main() {
    use crate::print_things::*;
    
    let my_billy = Billy::new(3);
    my_billy.print_billy();
}
