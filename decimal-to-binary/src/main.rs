use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");

    let decimal: u32 = input.trim().parse().expect("Invalid number");

    decimal_to_binary(decimal);
}

fn decimal_to_binary(mut decimal: u32) {
    while decimal > 0 {
        let remainder = decimal % 2;
        print!("{}", remainder);
        decimal = decimal / 2;
    }
}
