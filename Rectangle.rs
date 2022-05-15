use std::io;

fn main() {
    println!("Enter the length and width of the rectangle:"); 
    let mut length = String::new();
    let mut width = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");
    
    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line"); 

    let length: i32 = length.trim().parse().unwrap();
    let width: i32 = width.trim().parse().unwrap();
    
    println!("Area: {}", get_area(length, width));
    println!("Perimeter: {}", get_perimeter(length, width));
    println!("Vertical orientation: {}", vertical_orientation(length, width));
}

fn get_area(width: i32, length: i32) -> i32 {
    width * length
}

fn get_perimeter(width: i32, length: i32) -> i32 {
    2 * (width + length)
}

fn vertical_orientation(width: i32, length: i32) -> bool {
    width > length
}