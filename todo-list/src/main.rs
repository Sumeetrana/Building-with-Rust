use std::io;

#[derive(Debug)]
struct Task {
    description: String,
    priority: u8,
    completed: bool,
}

impl Task {}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();

    loop {
        let mut choice = String::new();
        println!("Please enter your choice:");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View tasks");
        println!("4. Edit task");
        println!("5. View completed tasks");
        println!("6. Complete task");
        println!("7. View pending tasks");
        println!("8. Exit");

        io::stdin().read_line(&mut choice).expect("Invalid input");

        let choice: u16 = choice.trim().parse().expect("Invalid number");

        match choice {
            1 => add_task(&mut task_list),
            2 => remove_task(&mut task_list),
            3 => view_tasks(&task_list),
            4 => edit_task(&mut task_list),
            5 => view_completed_tasks(&task_list),
            6 => complete_task(&mut task_list),
            7 => view_pending_tasks(&task_list),
            8 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn view_completed_tasks(task_list: &Vec<Task>) {
    for task in task_list {
        if task.completed {
            println!("{:?}", task);
        }
    }
}

fn view_pending_tasks(task_list: &Vec<Task>) {
    for task in task_list {
        if !task.completed {
            println!("{:?}", task);
        }
    }
}

fn complete_task(task_list: &mut Vec<Task>) {
    let mut task_number = String::new();

    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid number");

    let task_number: usize = task_number.trim().parse().expect("Invalid number");

    match task_list.get_mut(task_number - 1) {
        Some(value) => {
            value.completed = true;
        }
        None => println!("Invalid task_number"),
    }
}

fn add_task(task_list: &mut Vec<Task>) {
    let mut task_decription = String::new();
    println!("Please enter a description for your task description: ");
    io::stdin()
        .read_line(&mut task_decription)
        .expect("Invalid input");

    let mut task_priority = String::new();
    println!("Please enter your task priority: ");
    io::stdin()
        .read_line(&mut task_priority)
        .expect("Invalid input");

    task_list.push(Task {
        description: task_decription,
        priority: task_priority.trim().parse::<u8>().expect("Invalid value"),
        completed: false,
    });
}

fn remove_task(task_list: &mut Vec<Task>) {
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

fn view_tasks(task_list: &Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }

    println!("Task list: {:?}", task_list);
}

fn edit_task(task_list: &mut Vec<Task>) {
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

    let mut task_description = String::new();
    io::stdin()
        .read_line(&mut task_description)
        .expect("Invalid number");

    let mut task_priority = String::new();
    io::stdin()
        .read_line(&mut task_priority)
        .expect("Invalid number");

    task_list.remove(task_number - 1);
    task_list.insert(
        task_number - 1,
        Task {
            description: task_description,
            priority: task_priority.trim().parse::<u8>().expect("Invalid value"),
            completed: false,
        },
    );
}
