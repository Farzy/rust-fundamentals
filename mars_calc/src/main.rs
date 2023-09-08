use std::io;
use std::io::Write;

fn main() {
    print!("Enter your weight (kg): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().expect("expected a number");

    let mut mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight); // Use "cargo expand" to expand the macro
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
