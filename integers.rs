use std::fmt;
use std::ops::Add;

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp,
        }
    }
}

impl Add for Country {
    type Output = Self; // If you add two `Country` instances, the result will be another `Country`
                        // instance
    fn add(self, other: Self) -> Self {
        Self {
            name: format!("{} and {}", self.name, other.name),
            population: self.population + other.population,
            gdp: self.gdp + other.gdp,
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "In {} are {} people and a GDP of ${}",
            self.name, self.population, self.gdp
        )
    }
}

fn main() {
	let nauru = Country::new("Nauru", 10_670, 160_000_000);
    let vanuatu = Country::new("Vanuatu", 307_815, 820_000_000);
    let micronesia = Country::new("Micronesia", 104_468, 367_000_000);

	println!("{}", nauru.clone());
	println!("{}", nauru.clone() + vanuatu.clone());
	println!("{}", nauru + vanuatu + micronesia);
}
