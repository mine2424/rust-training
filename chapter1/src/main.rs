use std::time;

fn main() {
    // println! is a macro. "!" means it's a macro
    // macro is a function that generates code
    // and macro is expanded before the code is compiled
    println!("let's start rust training!");
    println!();

    // number
    let int_sum = 35 / 3;
    let real_sum = 35.0 / 3.0;
    println!("{} {}", int_sum, real_sum);
    println!();

    // FizzBuzz question in Rust
    fizz_buzz(5000);
    println!();

    // display nine_cross_nine_sheet
    nine_cross_nine_sheet();
    println!();

    // calculate_shopping_items
    calculate_shopping_items();
    println!();

    // caesar cipher
    let text = "HELLO";
    let shift = 3;
    let encrypted = caesar_encrypt(text, shift);
    println!("{} -> {}", text, encrypted);
    println!();

    // get prime numbers
    // & means reference variable (pointer) -> address
    // * means dereference variable (pointer) -> real value
    let mut primes = [0; 100];
    get_prime(&mut primes);
    // :? means debug print variable of data structure
    println!("{:?}", primes);
    println!();

    // reference test
    // & means reference variable (pointer) -> address!!!!!!!!!!!!
    // * means dereference variable (pointer) -> real value!!!!!!!!!!!
    let mut test_value = 10;
    reference_test(&mut test_value);
    println!("test_value: {}", test_value);
    println!("---------------- done -----------------");
}

fn start_timer() -> time::Instant {
    let now = time::Instant::now();
    println!("start_timer: {:?} seconds", now);
    return now;
}

fn end_timer(start_time: time::Instant) {
    let now = time::Instant::now();
    println!("end_time: {:?} seconds", now);
    println!("elapsed: {:?} seconds", start_time.elapsed());
}

fn fizz_buzz(max: i32) {
    let start_time = start_timer();
    if max < 1 {
        println!("max must be greater than 0");
        return;
    }
    // variable declaration is immutable by default
    // should use "mut" to make it mutable
    // should declare snake_case variable name
    let mut fizz_buzz_counts: i32 = 0;
    let mut fizz_counts = 0;
    let mut buzz_counts = 0;
    let mut other_counts = 0;
    // python style for loop
    for i in 1..max + 1 {
        if i % 3 == 0 && i % 5 == 0 {
            fizz_buzz_counts += 1;
        } else if i % 3 == 0 {
            fizz_counts += 1;
        } else if i % 5 == 0 {
            buzz_counts += 1;
        } else {
            other_counts += 1;
        }
        if i == max {
            println!("FizzBuzz: {}", fizz_buzz_counts);
            println!("Fizz: {}", fizz_counts);
            println!("Buzz: {}", buzz_counts);
            println!("Other: {}", other_counts);
            println!("Done {} counts", i);
        }
    }
    end_timer(start_time);
}

fn nine_cross_nine_sheet() {
    // for i in 1..10 {
    //     for j in 1..10 {
    //         if j == 9 {
    //             print!("{}", i * j);
    //             continue;
    //         } else {
    //             print!("{},", i * j);
    //         }
    //     }
    //     println!();
    // }

    // using macro
    for y in 1..10 {
        let s = (1..10).map(|x| format!("{:3}", x * y)).collect::<String>();
        println!("{}", s)
    }
}

fn calculate_shopping_items() {
    let pc_price = 98000.0;
    let send_fee_in_a = 1200.0;

    // f64 is a float type and pc_price should set f64 type
    let pc_price_in_a = (pc_price * 0.8) + send_fee_in_a;
    let pc_price_in_b = pc_price * 0.9;
    println!("pc_price_in_a: {}", pc_price_in_a);
    println!("pc_price_in_b: {}", pc_price_in_b);
}

// '' means char type and "" means string type
// '' can convert to i16 and "" can't convert to i16
fn caesar_encrypt(text: &str, shift: i16) -> String {
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    // a to z -> 65 to 90
    println!("code_a: {}", code_a);
    println!("code_z: {}", code_z);

    let mut result = String::new();

    for ch in text.chars() {
        // convert text code
        let mut code = ch as i16;
        if code_a <= code && code <= code_z {
            println!("{} -> {}", ch, code);
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        // convert code to text
        result.push(code as u8 as char);
    }

    return result;
}

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn get_prime(primes: &mut [usize; 100]) -> &mut [usize; 100] {
    let mut i = 2;
    let mut count = 0;

    // count reach to 100
    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
    return primes;
}

fn reference_test(test_value: &mut u32) {
    *test_value += 10;
}
