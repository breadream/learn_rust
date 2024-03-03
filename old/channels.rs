use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (sender, receiver) = channel();
    let hugevec = vec![0; 1_000_000];
    let mut newvec = vec![];
    let mut handle_vec = vec![];

    for i in 0..10 {
        let sender_clone = sender.clone();
        let mut work: Vec<u8> = Vec::with_capacity(hugevec.len() / 10);
        work.extend(&hugevec[i*100_000..(i+1)*100_000]);
        let handle = spawn(move || {
            for number in work.iter_mut() {
                *number += 1;
            }
            sender_clone.send(work).unwrap();
        });
        handle_vec.push(handle);
    }

    for handle in handle_vec {
        handle.join().unwrap();
    }

    while let Ok(results) = receiver.try_recv() {
        newvec.push(results);
    }

    let newvec = newvec.into_iter().flatten().collect::<Vec<u8>>();

    println!("{:?}, {:?}, total_length: {}", 
        &newvec[0..10], &newvec[newvec.len()-10..newvec.len()], newvec.len()
    );

    for number in newvec {
        if number != 1 {
            panic!();
        }
    }
}
