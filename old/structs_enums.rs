#[derive(Debug)] // to use {:?}
struct Animal {
	age: u8,
	animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
	Cat,
	Dog,
}

impl Animal {
    fn new() -> Self {
        // Self means Animal
        // You can also write Animal instead of Self
        Self {
            // When we write Animal::new(), we always get a dog that is 2 years old
            age: 10,
            animal_type: AnimalType::Dog,
        }
    }

    fn change_to_cat(&mut self) {
        // becase we are inside Animal, &mut self means &mut Animal use .change_to_cat()
        // to change the dog to a cat with &mut self we can change it
        println!("Changing animal to cat!");
        self.animal_type = AnimalType::Cat;
    }

    fn change_to_dog(&mut self) {
        // becase we are inside Animal, &mut self means &mut Animal use .change_to_cat()
        // to change the dog to a cat with &mut self we can change it
        println!("Changing animal to dog!");
        self.animal_type = AnimalType::Dog;
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
}

fn main() {
    let mut new_animal = Animal::new(); // Associated function to create a new animal
    new_animal.check_type();

    new_animal.change_to_cat();
    new_animal.check_type();

    new_animal.change_to_dog();
    new_animal.check_type();
}
