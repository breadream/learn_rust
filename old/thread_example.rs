use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let num_threads = 4;

    let chunk_size = (numbers.len() + num_threads - 1) / num_threads;

    let mut handles = vec![];

    for (i, chunk) in numbers.chunks(chunk_size).enumerate() {
        let chunk = chunk.to_vec();

        println!("Spawning thread {} with chunk: {:?}", i+1, chunk);
        
        let handle = thread::spawn(move || {
            println!("Thread processing chunk: {:?}", chunk);
            let sum: i32 = chunk.iter().sum();
            println!("Thread finished processing sum: {}", sum);
            sum
        });

        handles.push(handle)
    }

    let final_sum: i32 = handles
        .into_iter()
        .map(|handle| {
            println!("Waiting for a thread to finish...");
            let sum = handle.join().unwrap();
            println!("A thread has finished with sum: {}", sum);
            sum
        })
        .sum();
    

    println!("The sum of the number is : {}", final_sum);
}
