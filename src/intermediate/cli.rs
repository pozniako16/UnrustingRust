/*
The next exercise is:

### Exercise 4: Work with Collections and Ownership

- Build a program that manages a list of tasks (strings) using a `Vec<String>`.
- Implement functions to add a task, remove a task by index, and list all tasks.
- Create a command-line interface using `match` expressions to handle user commands such as `"add"`, `"remove"`, `"list"`, and `"exit"`.
- Manage ownership properly when passing and modifying the collection between functions.

This exercise will help you practice:

- Working with collections (`Vec`).
- Ownership and borrowing rules with mutable references.
- Using `match` expressions for control flow.
- Building a basic interactive CLI.

 */
use std::io;

enum Commands {
    ADD = 1,
    REMOVE = 2,
    LIST = 3,
    EXIT = 4
}

const AVAILABLE_COMMANDS: [(Commands, &str); 4] = [
    (Commands::ADD, "Add a new task."), 
    (Commands::REMOVE, "Remove a task."),
    (Commands::LIST, "List all tasks."),
    (Commands::EXIT, "Exit task manager"),
]; 

fn print_commands() {
    println!("What do you want to do ?");
    for idx in (0..AVAILABLE_COMMANDS.len()) {
        println!("{} - {}", idx+1, AVAILABLE_COMMANDS[idx].1);
    }
}

fn  read_user_input() -> String {
    let mut input = String::new(); // Create a mutable String to store the input
    io::stdin()
        .read_line(&mut input) // Read input from the user
        .expect("Failed to read line"); // Handle potential errors
    input
}

fn app() {
    loop {
        print_commands();
        
        let user_input = read_user_input();
        match user_input.parse::<usize>(){
            Ok(val) => {
                if val > AVAILABLE_COMMANDS.len() {
                    println!("Command with index {} does not exist.", val)
                }  
                
            }
            Err(err) => {println!("Cannot parse {} not a number. Detailed error: {}", user_input, err)}
        }
    }
}

pub fn cli() {
    println!("Welcome to task manager, what do you want to do ?");
    print_commands()
}
