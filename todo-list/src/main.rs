use std::io;

fn main() {
    let mut task_list: Vec<String> = Vec::new();

    loop {
        let mut choice = String::new();
        println!("Please enter your choice:");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View tasks");
        println!("4. Exit");

        io::stdin().read_line(&mut choice).expect("Invalid input");

        let choice: u16 = choice.trim().parse().expect("Invalid number");

        match choice {
            1 => add_task(&mut task_list),
            2 => remove_task(&mut task_list),
            3 => view_tasks(&task_list),
            4 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn add_task(task_list: &mut Vec<String>) {
    let mut decription = String::new();
    println!("Please enter a description for your task: ");
    io::stdin()
        .read_line(&mut decription)
        .expect("Invalid input");

    task_list.push(decription);
}
fn remove_task(task_list: &mut Vec<String>) {}
fn view_tasks(task_list: &Vec<String>) {}
