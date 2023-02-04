use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Current number is {}", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 1..5 {
        println!("Current number is {}", i);
        thread::sleep(Duration::from_millis(200));
    }
    handle.join().unwrap();
}
