use std::io;

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// Nested loops
// fn main() {
//     let mut count = 0;

//     'counting_up: loop {
//         println!("Count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("Remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }

//             if count == 2 {
//                 break 'counting_up;
//             }

//             remaining -= 1;
//         }

//         count += 1;
//     }

//     println!("End count = {count}");
// }

// While loop
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!")
// }

// Looping through every elements in an array
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("The value is {}, while index is {}", a[index], index);

//         index += 1
//     }
// }

// For loop
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// Countdown with for loop
// fn main() {
//     for element in (1..4).rev() {
//         println!("{element}");
//     }

//     println!("LIFTOFF!!!");
// }

// Convert from Celsius to fahrenheit
// fn main() {
//     println!("Enter a number");

//     let mut number = String::new();

//     io::stdin()
//         .read_line(&mut number)
//         .expect("Failed to read number");

//     let number: f64 = number.trim().parse().expect("Number not valid");

//     let result = (number * 1.8) + 32.0;

//     println!("{number} Celsius is equal to {result} fahrenheit");
// }

fn main() {
    println!("Enter the number of terms: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Cannott read number");

    let number: i32 = number.trim().parse().expect("Error");
    let mut index = 0;
    let mut first_term = 0;
    let mut second_term = 1;
    let mut next_term = 0;

    while index < number - 1 {
        next_term = first_term + second_term;
        first_term = second_term;
        second_term = next_term;

        index += 1
    }

    println!(
        "The {}th term of the Fibonacci series is: {}",
        index, next_term
    );
}

// using recursion
// fn main() {
//     println!("Enter the number of terms: ");
//     let mut number = String::new();

//     io::stdin()
//         .read_line(&mut number)
//         .expect("Cannott read number");

//     let number: u64 = number.trim().parse().expect("Error");

//     let result = fibonacci(number);

//     println!(
//         "The {}th term of the Fibonacci series is: {}",
//         number, result
//     );
// }

// fn fibonacci(number: u64) -> u64 {
//     if number < 2 {
//         return number;
//     }

//     return fibonacci(number - 1) + fibonacci(number - 2);
// }
