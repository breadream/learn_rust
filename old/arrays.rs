fn main() {
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];

    for city in my_cities {
        println!("{}", city);
    }
    for city in &my_cities {
        println!("{}", city);
    }
    for city in my_cities.iter() {
        println!("{}", city);
    }
    let [city1, city2, city3] = my_cities;
    println!("{}", city1);
}
