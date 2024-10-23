use std::{f32::consts::PI, io};

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

    println!("Enter the radius of a circle.");

    let mut radius = String::new();
    io::stdin()
        .read_line(&mut radius)
        .expect("Failed to read line");

    let radius: f32 = match radius.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    
    let area = radius.powf(2.0) * PI;

    println!("Area of the circle is {}", area)

}
// Task 2.3
pub fn area_of_rectangle() {
    println!("Enter the base of a rectangle.");

    let mut base = String::new();
    io::stdin()
        .read_line(&mut base)
        .expect("Failed to read line");

    let base: f32 = match base.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    
    println!("Enter the height of a rectangle.");

    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");

    let height: f32 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let border: f32 = base * 2.0 + height * 2.0; 
    let area: f32 = base * height;

    println!("Area of the rectangle is {} and the border is {}", area, border);
}