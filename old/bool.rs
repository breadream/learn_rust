fn main() {
    let bool_vec = vec![true, false, true, false, false];

    let option_vec = bool_vec
        .iter()
        .map(|item| {
            item.then(|| {
                println!("Got a {}!", item);
                "It's true, you know"
            })
        })
        .collect::<Vec<_>>();

    println!("Now we have: {:?}", option_vec);
    let filtered_vec = option_vec.into_iter().filter_map(|c| c).collect::<Vec<_>>();

    println!("And without the Nones: {:?}", filtered_vec);
}
