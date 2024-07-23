use std::collections::LinkedList;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {

    let list = Arc::new(Mutex::new(LinkedList::<i32>::new())); 

    let list_clone = Arc::clone(&list);

    thread::spawn(move || {
        for i in 1..=10{
            let mut list = list_clone.lock().unwrap();
            list.push_back(i*10);
            println!("Added: {}",i*10);
            thread::sleep(Duration::from_secs(1));
        }
    }).join().unwrap();

    let list = list.lock().unwrap();

    println!("Final list: {:?}", *list);
}
