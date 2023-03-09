struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // `y` gets partially moved during this pattern match. We can avoid this by using the `ref`
        // keyword to borrow the value instead of moving it. We need to do that here since we use
        // `y` later
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y;
}
