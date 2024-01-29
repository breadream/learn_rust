struct Animal {
    name: String,
}

trait Dog {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("The dog is running")
    }
}

impl Dog for Animal {
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
}
