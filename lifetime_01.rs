#[derive(Debug)]
struct City<'city> {
    name: &'city str,
    date_founded: u32,
}

fn main() {
    let city_names = vec!["Seoul".to_string(), "Seattle".to_string()]; 
    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}
