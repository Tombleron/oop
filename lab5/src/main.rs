use std::io;

fn main() {
    let my_array = [10, 20, 30, 40, 50];

    println!("Enter the index to get the element:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid index.");
            return;
        }
    };

    match my_array.get(index) {
        Some(value) => {
            println!("Element at index {} is: {}", index, value);
        }
        None => {
            println!("Index out of bounds. Please enter a valid index.");
        }
    }
}
