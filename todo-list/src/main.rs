use std::io;

fn main() {
    let mut task_list: Vec<String> = Vec::new();

    loop {
        let mut choice = String::new();
        println!("Please enter your choice:");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View tasks");
        println!("4. Edit task");
        println!("5. Exit");

        io::stdin().read_line(&mut choice).expect("Invalid input");

        let choice: u16 = choice.trim().parse().expect("Invalid number");

        match choice {
            1 => add_task(&mut task_list),
            2 => remove_task(&mut task_list),
            3 => view_tasks(&task_list),
            4 => edit_task(&mut task_list),
            5 => {
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

    task_list.push(decription.trim().to_string());
}

fn remove_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("NO tasks are found");
        return;
    }

    println!("Please enter the task no. which you want to remove");
    view_tasks(task_list);

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid choice");

    let task_number: usize = choice.trim().parse().expect("Invalid choice");
    task_list.remove(task_number - 1);
    println!("Task is removed");
}

fn view_tasks(task_list: &Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }

    println!("Task list: {:?}", task_list);
}

fn edit_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }

    println!("Please enter the task number that you want to edit:");
    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid number");
    let task_number: usize = task_number.trim().parse().expect("Invalid number");
    println!("Please enter new task:");

    let mut task_name = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("Invalid number");

    task_list.remove(task_number - 1);
    task_list.insert(task_number - 1, task_name.trim().to_string());
}
