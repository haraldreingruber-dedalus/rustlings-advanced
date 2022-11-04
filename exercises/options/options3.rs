// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(_) => println!("Co-ordinates are {},{} ", 10, 10),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
