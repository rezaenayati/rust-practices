use std::io;

fn main() {
    println!("==========================================================");
    println!("Generate the nth Fibonacci number.");
    println!("Please Enter n:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Error happened");

    let mut index = 0;

    let n: i32 = n.trim().parse().expect("Error happened");

    let mut future_value: i32 = 1;
    let mut current_value: i32 = 0;

    while index < n {
        let temp: i32 = future_value;
        future_value = current_value + future_value;
        current_value = temp;

        index = index + 1;
    }

    println!("{}th number of Fibonacci is {}", n, current_value);

    println!("==========================================================");
}
