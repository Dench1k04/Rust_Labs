fn is_valid_brackets(s: &str) -> bool {
    let mut stack = Vec::new();
    
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => continue, 
        }
    }
    
    stack.is_empty() 
}

fn clean_phone_number(phone: &str) -> String {
    let mut result = String::new();
    
    for ch in phone.chars() {
        if result.is_empty() && ch == '+' {
            continue;
        }
        if ch.is_digit(10) {
            if result.is_empty() && ch == '3' && phone.starts_with("+3") {
                continue;
            }
            result.push(ch);
        }
    }
    
    result
}

fn main() {
    println!("Перевірка дужок:");
    let test_cases = vec![
        "([]{})[]",   // true
        "([]]",      // false
        "{[()]}",    // true
        "([)]",      // false
        "hello(world)[]{}", // true
        "",          // true
        "(",         // false
    ];
    
    for test in test_cases {
        println!("\"{}\" -> {}", test, is_valid_brackets(test));
    }
    
    println!("\nОчищення телефонних номерів:");
    let phone_numbers = vec![
        "+3 (050)-995-0253",
        "050-995-0253",
        "3 050 995 0253",
        "050.995.0253",
    ];
    
    for number in phone_numbers {
        let cleaned = clean_phone_number(number);
        println!("{} -> {}", number, cleaned);
    }
}