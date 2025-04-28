use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let mutex_vector = Mutex::new((0..20).collect::<Vec<_>>());
    let vector_size = mutex_vector.lock().unwrap().len();
    let handles: Vec<_> = (0..vector_size)
        .map(|i| {
            let mutex_vector = mutex_vector.lock().unwrap().clone();
            thread::spawn(move || {
                let mut vector = mutex_vector;
                thread::sleep(Duration::from_millis(rand::random::<u64>() % 3000));
                vector[i] += 1;
                println!("Thread {} incremented index {}: {}", i, i, vector[i]);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
