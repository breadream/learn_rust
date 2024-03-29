#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}


fn main() {
    let mut vec_integers = vec![1, 5, 10, 2, 15];
    let mut vec_floats = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec_integers.sort(); // or sort_unstable()
    vec_floats.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec_integers, vec![1, 2, 5, 10, 15]);
    assert_eq!(vec_floats, vec![1.1, 1.123, 2.0, 5.5]);

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort();
    
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]);


    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]);
}
