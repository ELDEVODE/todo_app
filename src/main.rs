use chrono::{DateTime, Local};
use std::io;

#[derive(Debug)]
struct Todo {
    id: u32,
    description: String,
    completed: bool,
    created_at: DateTime<Local>,
}

impl Todo {
    fn new(id: u32, description: String) -> Self {
        Todo {
            id,
            description,
            completed: false,
            created_at: Local::now(),
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true
    }
}

struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let id = self.todos.len() as u32 + 1;
        let task = Todo::new(id, description);
        self.todos.push(task)
    }

    fn list_tasks(&self) {
        for todo in &self.todos {
            println!("{:?}", todo)
        }
    }

    fn complete_task(&mut self, id: u32) -> Option<()> {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.mark_completed();
                return Some(());
            }
        }
        None
    }

    fn delete_tasks(&mut self, id: u32) -> Option<()> {
        if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
            self.todos.remove(pos);
            return Some(());
        }
        None
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let stdin = io::stdin();

    loop {
        println!("\nPlease choose an option:");
        println!("1. Add a task");
        println!("2. List tasks");
        println!("3. Complete a task");
        println!("4. Delete a task");
        println!("5. Exit");

        let mut choice = String::new();
        stdin.read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter the task description:");
                let mut task_input = String::new();
                stdin.read_line(&mut task_input).unwrap();
                let task_input = task_input.trim().to_string();
                todo_list.add_task(task_input)
            }

            "2" => {
                println!("Current Tasks:");
                todo_list.list_tasks();
            }

            "3" => {
                println!("Enter the task ID to complete:");
                let mut id_input = String::new();
                stdin.read_line(&mut id_input).unwrap();
                if let Ok(id) = id_input.trim().parse::<u32>() {
                    if todo_list.complete_task(id).is_some() {
                        println!("Task {} marked as completed.", id);
                    } else {
                        println!("Task with Id {} not found.", id);
                    }
                } else {
                    println!("Invalid ID entered.")
                }
            }
            "4" => {
                println!("Enter the task ID to delete:");
                let mut id_input = String::new();
                stdin.read_line(&mut id_input).unwrap();
                if let Ok(id) = id_input.trim().parse::<u32>() {
                    if todo_list.delete_tasks(id).is_some() {
                        println!("Task {} deleted.", id);
                    } else {
                        println!("Invalid ID entered.")
                    }
                }
            }
            "5" => break,
            _ => {
                println!("Invalid option. Please try again.")
            }
        }
    }
    println!("Goodbye!");
}
