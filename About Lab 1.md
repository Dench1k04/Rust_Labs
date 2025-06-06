# Пояснення  
> У цьому файлі я пояснюю, що виконується в програмі, написаній на мові Rust.

## Числа Фібоначчі — `fib(n: u32) -> Vec<u32>`

```rust
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
```

> Ця функція обчислює послідовність чисел Фібоначчі та повертає її у вигляді вектора.  
> Спочатку створюється порожній вектор `sequence`. Якщо `n == 0`, функція одразу повертає його.  
> Далі в послідовність додаються перші три значення: `0`, `1`, `1`, що не зовсім коректно для всіх `n`, але в цьому варіанті зроблено для прикладу.  
> Якщо `n == 1`, то знову повертається вже підготовлений вектор.  
> Якщо `n > 1`, починає працювати цикл, в якому кожен наступний елемент обчислюється як сума двох попередніх. Цикл працює `n-1` разів.  
> Фінально функція повертає повний вектор.

---

## Таблиця множення — `print_pythagorean_table()`

```rust
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
```

> Це функція, яка виводить таблицю множення.  
> Спочатку виводиться заголовок (числа від 1 до 10) із відступом у 3 пробіли (`{:3}`).  
> Далі кожен рядок починається з поточного числа `i`, вирівняного у 2 пробіли (`{:2}`), після чого йде `j` — множення кожного `i` на `j` у діапазоні від 1 до 10.  
> `println!()` переносить курсор на новий рядок після кожного рядка таблиці.

---

## Головна функція — `main()`

```rust
fn main() {
    let n = 5;
    let fib_result = fib(n);
    println!("Для n = {}: {:?}", n, fib_result);

    println!("\nКвадрат Піфагора:");
    print_pythagorean_table();
}
```

> У функції `main()` викликається обчислення послідовності Фібоначчі для `n = 5` і результат виводиться у консоль.  
> Після цього викликається функція `print_pythagorean_table()`, яка виводить таблицю множення.  
> Таким чином, програма виконує дві задачі: обчислення чисел Фібоначчі та виведення таблиці Піфагора.
