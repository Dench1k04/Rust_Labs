use std::thread;
use std::time::Instant;
use rand::Rng;
use std::sync::Arc;

fn fib(n: u32) -> Vec<u32> {
    let mut sequence = Vec::new();

    if n == 0 {
        return sequence;
    }

    sequence.push(0);
    if n == 1 {
        return sequence;
    }

    sequence.push(1);
    sequence.push(1);

    for _ in 1..n {
        let next = sequence[sequence.len() - 1] + sequence[sequence.len() - 2];
        sequence.push(next);
    }

    sequence
}

fn task1() {
    let handles: Vec<_> = (0..5).map(|i| {
        thread::spawn(move || {
            let start = Instant::now();
            let start_n = i * 5;
            let end_n = (i + 1) * 5;
            for n in start_n..end_n {
                let fib_result = fib(n);
                println!("Потік {}: n = {}, {:?}", i, n, fib_result);
            }
            let duration = start.elapsed();
            let ms = duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0;
            println!("Потік {} час: {:.2}мс", i, ms);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn sequential_processing(arr: &[i32]) -> i32 {
    let mut counts = [0; 301];
    for &num in arr {
        counts[(num + 200) as usize] += 1;
    }
    let mut max_count = 0;
    let mut max_num = 0;
    for i in 0..301 {
        if counts[i] > max_count {
            max_count = counts[i];
            max_num = i as i32 - 200;
        }
    }
    max_num
}

fn parallel_processing(arr: Arc<Vec<i32>>, m: usize) -> i32 {
    let chunk_size = arr.len() / m;
    let handles: Vec<_> = (0..m).map(|i| {
        let arr = Arc::clone(&arr);
        thread::spawn(move || {
            let slice = &arr[i * chunk_size..(i + 1) * chunk_size];
            let mut counts = [0; 301];
            for &num in slice {
                counts[(num + 200) as usize] += 1;
            }
            counts
        })
    }).collect();

    let mut total_counts = [0; 301];
    for handle in handles {
        let counts = handle.join().unwrap();
        for i in 0..301 {
            total_counts[i] += counts[i];
        }
    }

    let mut max_count = 0;
    let mut max_num = 0;
    for i in 0..301 {
        if total_counts[i] > max_count {
            max_count = total_counts[i];
            max_num = i as i32 - 200;
        }
    }
    max_num
}

fn task2() {
    let mut rng = rand::thread_rng();
    let mut arr = vec![0; 100000];
    for i in 0..100000 {
        arr[i] = rng.gen_range(-200..=100);

    }
    let arr_arc = Arc::new(arr);

    let start = Instant::now();
    let result_seq = sequential_processing(&arr_arc);
    let seq_time = start.elapsed();
    let seq_ms = seq_time.as_secs() as f64 * 1000.0 + seq_time.subsec_nanos() as f64 / 1_000_000.0;
    println!("Послідовний результат: {}, час: {:.2}мс", result_seq, seq_ms);

    for m in [2, 4, 8].iter() {
        let start = Instant::now();
        let result_par = parallel_processing(Arc::clone(&arr_arc), *m);
        let par_time = start.elapsed();
        let par_ms = par_time.as_secs() as f64 * 1000.0 + par_time.subsec_nanos() as f64 / 1_000_000.0;
        println!("Паралельний (m={}): {}, час: {:.2}мс", m, result_par, par_ms);
    }
}

fn main() {
    println!("Завдання 1:");
    task1();
    println!("\nЗавдання 2:");
    task2();
}