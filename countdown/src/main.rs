use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");

        // let timer: u16 = input.trim().parse().expect("Invalid number");

        let timer: u16 = match input.trim().parse() {
            Ok(timer) => timer,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        start_timer(timer);
        break;
    }
}

fn start_timer(timer: u16) {
    for i in (1..=timer).rev() {
        println!("Timer countdown...{}", i);
        sleep(Duration::from_secs(1));
    }
}
