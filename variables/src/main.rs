use std::io;

// fn main() {
//     let mut x = 5;

//     println!("The value of x is: {x}");

//     x = 6;

//     println!("the value of x is now: {x}");
// }

// Variable Shadowing
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// Tuple 
// fn main() {
//     let tup: (f32, u8, i32) = (5.0, 27, 100);
// }

// Destructuring tuple values
// fn main() {
//     let tup = (50, 9, 10.5);

//     let (x, y, z) = tup;

//     println!("The value of x is: {x}");
//     println!("The value of y is: {y}");
//     println!("The value of z is: {z}");
// }

// Getting the value of tuple using the dot notation
// fn main() {
//     let tup: (i64, f32, u8) = (84, 7.7, 5);

//     let x = tup.0;
//     let y = tup.1;
//     let z = tup.2;

//     println!("The value of x is: {x}");
//     println!("The value of y is: {y}");
//     println!("The value of z is: {z}");
// }

// Arrays
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     // An array with specified data type and length
//     let a: [i32; 5] = [1, 2, 3, 4, 5];

//     // An array with the same value and length
//     let a = [3; 5];
// }

// Guessing game with Array
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: usize = guess.trim().parse().expect("Please enter a guess");

    let element = a[guess];

    println!("The value of the element at index {guess} is: {element}");
}