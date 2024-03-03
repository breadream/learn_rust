fn main() {
    let new_vec = vec![8, 9, 0];

    let number_to_add = 5;
    let mut empty_vec = vec![];

    for index in 0..5 {
        empty_vec.push )
            new_vec
                .get(index)
                .and_then(|number| Some(number + 1))
                .and_then(|number| Some(number + number_to_add))
        );
    }
    println!("{:?}", empty_vec);
}
