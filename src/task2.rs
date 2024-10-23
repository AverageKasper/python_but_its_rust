use std::{f32::consts::PI, io};

use rand::Rng;

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
// Task 2.5
pub fn talent_calculation() {
    let talents: f32 = float_input_check("Enter talents");
    let pounds: f32 = float_input_check("Enter pounds");
    let lots: f32 = float_input_check("Enter lots");

    let lot_weight: f32 = 13.3;
    let pound_weight: f32 = lot_weight * 32.0;
    let talent_weight: f32 = pound_weight * 20.0;

    let total_weight: f32 = talents * talent_weight + pounds * pound_weight + lots * lot_weight;
    let kilogram: f32 = total_weight / 1000.0;
    let gram: f32 = total_weight - kilogram.floor() * 1000.0;

    println!(
        "Total weight: {} kilograms, {:.2} grams.",
        kilogram.floor(),
        gram
    )
}
// Task 2.6
pub fn combinations() {
    fn math(num: i32, random: i32) -> i32 {
         num * 10 + random
    }
    let mut rng = rand::thread_rng();
    let mut comb1: i32 = 0;
    let mut comb2: i32 = 0;
    for _ in 1..=3 {
        let num = math(comb1,rng.gen_range(0..=9));
        comb1 = num;
    }
    for _ in 1..=4 {
        let num = math(comb2,rng.gen_range(1..=6));
        comb2 = num;
    }
    

    println!("3-digit code is {} and 4 digit code is {}", comb1, comb2)
}