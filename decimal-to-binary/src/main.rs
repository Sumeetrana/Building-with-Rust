use std::any::type_name;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");

    let decimal: u32 = input.trim().parse().expect("Invalid number");

    decimal_to_binary(decimal);
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn decimal_to_binary(decimal: u32) {
    let mut binary = String::from("");
    let mut temp = decimal;

    loop {
        let value = temp / 2;
        println!("{} {} {}", value, type_of(&value), value % 2);

        binary += &(value % 2).to_string();

        if temp == 0 {
            break;
        }
        temp = value / 2;
    }

    println!("{}", binary);
}
