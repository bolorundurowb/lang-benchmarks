use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::thread::available_parallelism;


fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true  
}

fn main() {
    let max = i32::MAX as u64;
    let num_threads = available_parallelism().unwrap().get() as u64;
    let chunk_size = (max / num_threads) + 1;

    let prime_count = Arc::new(Mutex::new(0i32));
    let mut handles = vec![];

    let start = Instant::now();

    for i in 0..num_threads {
        let prime_count = Arc::clone(&prime_count);
        let start_range = i * chunk_size;
        let end_range = if i == num_threads - 1 {
            max
        } else {
            (i + 1) * chunk_size
        };

        let handle = thread::spawn(move || {
            for n in start_range..end_range {
                if is_prime(n) {
                    let mut prime_count = prime_count.lock().unwrap();
                    *prime_count += 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let prime_count = prime_count.lock().unwrap();
    println!("Found {} primes in {:?} milliseconds", prime_count, duration.as_millis());
}
