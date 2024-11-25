// fn main() {
//     println!("Hello, world!");

//     another_function(10);
// }

// fn another_function() {
//     println!("Another function");
// }

// Function with Parameters
// fn main() {
//     another_function(10);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// Function with multiple Paramters
// fn main() {
//     print_labelled_measurement(10, 'm');
// }

// fn print_labelled_measurement(value: i32, unit: char) {
//     println!("The labelled measurement is: {value}{unit}")
// }

// function that returns a value
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
