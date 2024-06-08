use std::thread;

pub fn main() {
    let data = vec![1, 2, 3];
    let cloned_data = data.clone();

    // Create a closure that captures `data` by value
    let handle = thread::spawn(move || {
        println!("Captured data by value: {:?}", cloned_data);
    });

    // Wait for the thread to finish
    handle.join().unwrap();
    println!("{:?}", data);

    // `data` is no longer accessible here because it was moved into the closure
    // println!("{:?}", data); // This would cause a compile-time error
}