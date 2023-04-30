use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];

    let result = multiply_matrices(&a, &b);
    println!("Result: {:?}", result);

    let shared_value = Arc::new(Mutex::new(0));

    let shared_value_a = Arc::clone(&shared_value);
    let shared_value_b = Arc::clone(&shared_value);

    let start = Instant::now();

    let thread_a = thread::spawn(move || {
        for _ in 0..1_000_000_000 {
            let mut value = shared_value_a.lock().unwrap();
            *value += 1;
        }
    });

    let thread_b = thread::spawn(move || {
        for _ in 0..1_000_000_000 {
            let mut value = shared_value_b.lock().unwrap();
            *value += 1;
        }
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    let elapsed = start.elapsed();

    let final_value = *shared_value.lock().unwrap();
    println!("Final value: {}", final_value);
    println!("Elapsed time: {:?}", elapsed);
}

fn multiply_matrices(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = a[0].len();
    let k = b[0].len();

    let mut result = vec![vec![0; k]; n];

    let threads: Vec<_> = (0..n)
        .flat_map(|i| (0..k).map(move |j| (i, j)))
        .map(|(i, j)| {
            thread::spawn(move || {
                for t in 0..m {
                    result[i][j] += a[i][t] * b[t][j];
                }
                println!("Thread ({}, {}) calculated value: {}", i, j, result[i][j]);
            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap();
    }

    result
}
