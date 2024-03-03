use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}

fn main() {
    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000,
        city_history: Rc::new("Calgary has what..?".to_string()),
    };

    let canada_cities = CityData {
        names: vec![calgary.name],
        histories: vec![calgary.city_history.clone()],
    };

    println!("calgary's history is : {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));
    let new_owner = calgary.city_history.clone();
    println!("{}", Rc::strong_count(&calgary.city_history));
}
