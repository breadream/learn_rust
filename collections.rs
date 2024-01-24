use std::collections::BTreeMap; // HashMap - unsorted

struct City {
    name: String,
    year_population: BTreeMap<u32, u32>,
}

fn main() {
    let mut seattle = City {
        name: "Seattle".to_string(),
        year_population: BTreeMap::new(),
    };

    seattle.year_population.insert(1392, 3_250);
    seattle.year_population.insert(1900, 4_244);
    seattle.year_population.insert(2011, 131_123);

    for (year, population) in seattle.year_population {
        println!("In the year {} the city of {} had a population of {}", year, seattle.name, population); 
    }
}
