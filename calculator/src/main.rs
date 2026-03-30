use std::io;

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    if a > b {
        return a - b;
    }
    return b - a;
}

fn div(a: u32, b: u32) -> Option<f64> {
    if b == 0 {
        return None;
    }
    return Some((a as f64) / (b as f64));
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}

fn square_root(a: u32) -> f64 {
    (a as f64).sqrt()
}

fn get_number(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);

        io::stdin().read_line(&mut input).expect("Not valid input");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Try again"),
        }
    }
}

fn main() {
    let num_one = get_number("Enter your first number");
    let num_two = get_number("Enter your second number");

    println!("This addition is {}", add(num_one, num_two));
    println!("This subtraction is {}", sub(num_one, num_two));
    println!("This multiplication is {}", mul(num_one, num_two));

    match div(num_one, num_two) {
        Some(result) => println!("The division result is {}", result),
        None => println!("Division error: The denominator is zero"),
    }
}
