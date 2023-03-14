// The Clippy tool is a collection of lints to analyze your code so you can catch common mistakes
// and improve your Rust code.

use std::f32;

fn main() {
    // Use the constant PI provided by `std::f32`
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
