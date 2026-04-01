use std::io;

fn main() {
    let mut balance: u32 = 0;
    println!("Welcome to out simple banking application");

    loop {
        println!("Choose below options: ");
        println!("1. Deposit money");
        println!("2. Withdraw money");
        println!("3. Check user current balance");
        println!("4. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Not a valid input");

        let option: u16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again");
                continue;
            }
        };

        match option {
            1 => {
                println!("Enter money to depost: ");
                let mut deposit_input = String::new();
                io::stdin()
                    .read_line(&mut deposit_input)
                    .expect("Invalid input");
                let deposit_value: u32 = deposit_input.trim().parse().expect("Invalid input");
                balance += deposit_value;
                continue;
            }
            2 => {
                println!("Enter money to withdraw: ");
                let mut withdraw_input = String::new();
                io::stdin()
                    .read_line(&mut withdraw_input)
                    .expect("Invalid input");
                let withdraw_value: u32 = withdraw_input.trim().parse().expect("Invalid value");
                balance -= withdraw_value;
                continue;
            }
            3 => {
                println!("You balance is: {}", balance);
                continue;
            }
            4 => break,
            _ => {
                println!("Invalid option");
                continue;
            }
        }
    }
}
