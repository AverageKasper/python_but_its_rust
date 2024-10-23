mod task1;
mod task2;
use std::io;
use task1::hello_world;
use task2::{area_of_circle, area_of_rectangle, hello_user, number_solution};
fn main() {
    println!("Python but its Rust Tasks");
    println!("Type the number of the task you want to execute");
    println!("Task 1 - Hello World");
    println!("Task 2.1 - Hello User");
    println!("Task 2.2 - Area of a circle");
    println!("Task 2.3 - Area of a rectange");
    println!("Task 2.4 - 3 Number sum, product and average");
    println!("Task  - Placeholder");
    println!("Choose witch task to execute");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: f64 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    if choice == 1.0 {
        hello_world();
    } else if choice == 2.1 {
        hello_user();
    } else if choice == 2.2 {
        area_of_circle();
    } else if choice == 2.3 {
        area_of_rectangle();
    } else if choice == 2.4 {
        number_solution();
    }
}
