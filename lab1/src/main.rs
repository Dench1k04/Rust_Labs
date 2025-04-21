fn fib(n: u32) -> Vec<u32> {
    let mut sequence = Vec::new();
    
    if n == 0 {
        return sequence;
    }
    
    sequence.push(0);
    sequence.push(1);
    sequence.push(1);
    
    if n == 1 {
        return sequence;
    }
    
    for _ in 1..n {
        let next = sequence[sequence.len() - 1] + sequence[sequence.len() - 2];
        sequence.push(next);
    }
    
    sequence
}

fn print_pythagorean_table() {
    print!("  ");
    for i in 1..=10 {
        print!("{:3}", i);
    }
    println!();
    
    for i in 1..=9 {
        print!("{:2}", i);
        for j in 1..=10 {
            print!("{:3}", i * j);
        }
        println!();
    }
}

fn main() {
    let n = 5;
    let fib_result = fib(n);
    println!("Для n = {}: {:?}", n, fib_result);
    
    println!("\nКвадрат Піфагора:");
    print_pythagorean_table();
}