use std::collections::BinaryHeap;

fn main() {
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "Read book"));
    jobs.push((80, "Eat"));
    jobs.push((10, "Watch TV"));
    jobs.push((15, "Drink Whiskey"));

    while let Some(job) = jobs.pop() {
        println!("You need to : {}", job.1);
    }
}
