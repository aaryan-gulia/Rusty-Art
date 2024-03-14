use std::io::{self, Write};
use std::str::FromStr;
use bevy::prelude::*;

pub enum ShapeType {
    Circle { radius: f32 },
    Square { side_length: f32 },
    EquilateralTriangle { side_length: f32 },
    Rectangle { width: f32, height: f32 },
}

pub struct UserInput {
    pub shape: ShapeType,
}

impl Resource for UserInput {}

pub fn get_input() -> UserInput {
    let shape_type = prompt("Enter the type of shape (Circle, Square, EquilateralTriangle, Rectangle): ");
    match shape_type.as_str() {
        "Circle" => {
            let radius: f32 = prompt("Enter the radius of the circle: ").parse().unwrap();
            UserInput { shape: ShapeType::Circle { radius } }
        }
        "Square" => {
            let side_length: f32 = prompt("Enter the side length of the square: ").parse().unwrap();
            UserInput { shape: ShapeType::Square { side_length } }
        }
        "EquilateralTriangle" => {
            let side_length: f32 = prompt("Enter the side length of the equilateral triangle: ").parse().unwrap();
            UserInput { shape: ShapeType::EquilateralTriangle { side_length } }
        }
        "Rectangle" => {
            let width: f32 = prompt("Enter the width of the rectangle: ").parse().unwrap();
            let height: f32 = prompt("Enter the height of the rectangle: ").parse().unwrap();
            UserInput { shape: ShapeType::Rectangle { width, height } }
        }
        _ => panic!("Invalid shape type"),
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}