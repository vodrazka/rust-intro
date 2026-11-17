use std::thread;

pub fn main() {
    let data = vec![1, 2, 3];
    // Clone the vector so the original stays usable in this thread
    // PL: Klonujemy wektor, aby oryginał pozostał dostępny w tym wątku
    let cloned_data = data.clone();

    // Create a closure that captures `data` by value
    // PL: Tworzymy domknięcie (closure), które przechwytuje `data` przez wartość
    // `move` transfers ownership of `cloned_data` into the new thread
    // PL: `move` przenosi własność `cloned_data` do nowego wątku
    let handle = thread::spawn(move || {
        println!("Captured data by value: {:?}", cloned_data);
    });

    // Wait for the thread to finish
    // PL: Czekamy na zakończenie wątku
    handle.join().unwrap();
    // `data` is still accessible because only its clone was moved
    // PL: `data` jest nadal dostępne, ponieważ przeniesiony został tylko jego klon
    println!("{:?}", data);

    // If `data` itself had been moved into the closure, it would no longer be accessible here
    // PL: `data` nie byłoby tu już dostępne, gdyby zostało przeniesione do domknięcia
    // println!("{:?}", data); // This would cause a compile-time error / PL: To spowodowałoby błąd kompilacji
}
