mod country {
    fn print_country(country: &str) {
        println!("We are in the country of {}", country);
    }
    pub mod province {
        fn print_province(province: &str) {
            println!("in the province of {}", province);
        }

        pub mod city {
            use super::super::*;
            use super::*;

            pub fn print_city(country: &str, province: &str, city: &str) {
                print_country(country);
                print_province(province);
                println!("in the city of {}", city);
            }
        }
    }
}

fn main() {
    use crate::country::province::city::print_city;

    print_city("Canada", "New Brunswick", "Moncton");
    print_city("Korea", "Gyeonggi-do", "Gwangju");
}
