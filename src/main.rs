use std::sync::{Arc, Mutex};
// use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    {
        let counter = Arc::clone(&counter);
        let mut num = counter.lock().unwrap();
        *num += 2;
    }

    {
        let counter = Arc::clone(&counter);
        let mut num = counter.lock().unwrap();
        *num += 3;
    }

    {
        let counter = Arc::clone(&counter);
        let mut num = counter.lock().unwrap();
        *num += 4;
    }

    println!("{:?}", counter);
}