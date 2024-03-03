use std::collections::VecDeque;

fn check_remaining(input: &VecDeque<(&str, bool)>) {
    for item in input {
        if item.1 == false {
            println!("You must {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap();
    task_done.1 = true;
    input.push_front(task_done);
}

fn main() {
    let mut my_vecdq = VecDeque::new();
    let things_to_do = vec!["send email", "add new product", "play games"];

    for thing in things_to_do {
        my_vecdq.push_front((thing, false));
    }

    done(&mut my_vecdq);
    done(&mut my_vecdq);
    
    check_remaining(&my_vecdq);

    for task in my_vecdq {
        print!("{:?} ", task);
    }
}
