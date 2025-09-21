use chrono::{DateTime, Local};
use std::io;

#[derive(Debug)]
struct TodoStruct {
    id: u32,
    title: String,
    is_completed: bool,
    created_at: DateTime<Local>,
}
fn main() {
    // let todo1 = TodoStruct {
    //     id: 1,
    //     title: "Learn Rust",
    //     is_completed: false,
    //     is_created: Local::now(),
    // };
    let mut todos: Vec<TodoStruct> = Vec::new();
    // todos.push(todo1);

    // println!("{:?}", todos);
    loop {
        println!("press option to perform operation");
        println!("1. Create todo");
        println!("2. Show all todo");
        println!("3. Update todo");
        println!("4. Delete todo");
        println!("5. Exit");

        let mut option_string = String::new();
        io::stdin()
            .read_line(&mut option_string)
            .expect("Failed to read line");
        let option: u32 = option_string.trim().parse().unwrap_or(0);

        match option {
            1 => {
                create_todo(&mut todos);
                println!("Todo created successfully!");
            }
            2 => {
                println!("{:#?}", todos);
            }
            3 => {
                println!("update todos");
            }
            4 => {
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("failed to read id");
                let id = id.trim().parse().unwrap_or(0);
                delete_todo(&mut todos, id);
                println!("Todo of id {id} is successfully deleted!");
            }
            5 => break,
            _ => {
                println!("invalid option please choose correct options");
            }
        }
    }
}
fn create_todo(todos: &mut Vec<TodoStruct>) {
    let id = todos.len() as u32 + 1;
    let mut title = String::new();

    println!("Enter Title");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    let title = title.trim().to_string();

    let mut completed_input = String::new();
    println!("is completed");
    io::stdin()
        .read_line(&mut completed_input)
        .expect("Failed to read line");

    let is_completed = completed_input.trim().eq_ignore_ascii_case("true");
    todos.push(TodoStruct {
        id: id,
        title: title.trim().to_string(),
        is_completed: is_completed,
        created_at: Local::now(),
    });
}
fn delete_todo(todos: &mut Vec<TodoStruct>, id: u32) {
    if id == 0 {
        println!("Invalid ID. ID should be greater than 0");
    } else {
        todos
            .iter()
            .position(|todo| todo.id == id)
            .map(|index| {
                todos.remove(index);
            })
            .unwrap();
    }
}
