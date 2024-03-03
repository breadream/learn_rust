use std::collections::HashMap;

fn main() {
    let data = vec![
        ("male", 9),
        ("female", 11),
        ("male", 17),
        ("female", 8),
        ("male", 41),
        ("female", 94),
    ];

    let mut survey_hash = HashMap::new();

    for item in data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }

    for (gender, numbers) in survey_hash {
        println!("{:?} {:?}", gender, numbers);
    }
}
