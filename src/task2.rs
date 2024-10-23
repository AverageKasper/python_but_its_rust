use std::{
    f32::consts::PI,
    io,
};

// float input check
fn float_input_check(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
            }
        }
    }
}
// Task 2.1
pub fn hello_user() {
    println!("What is your name?");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    println!("Hello, {}!", name)
}

// Task 2.2
pub fn area_of_circle() {
    let radius: f32 = float_input_check("Enter the radius of a circle.");

    let area = radius.powf(2.0) * PI;

    println!("Area of the circle is {}", area)
}
// Task 2.3
pub fn area_of_rectangle() {
  
    let base: f32 = float_input_check("Enter the base of a rectangle.");
    
    let height: f32 = float_input_check("Enter the height of a rectangle.");

    let border: f32 = base * 2.0 + height * 2.0;

    let area: f32 = base * height;

    println!(
        "Area of the rectangle is {} and the border is {}",
        area, border
    );
}
// Task 2.4
pub fn number_solution() {
    let num_one: f32 = float_input_check("Enter the first number");

    let num_two: f32 = float_input_check("Enter the second number");

    let num_three: f32 = float_input_check("Enter the third number");

    let sum: f32 = num_one + num_two + num_three;

    let multi: f32 = num_one * num_two * num_three;

    let average: f32 = sum / 3.0;

    println!(
        "The sum is {}, product is {} and average is {}.",
        sum, multi, average
    )
}
