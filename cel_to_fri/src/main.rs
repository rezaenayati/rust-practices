use std::io;

fn main() {
    println!("=====================================================================");
    println!("A Rust Program to Convert Temperatures between Fahrenheit and Celsius");
    
    println!("Please Enter a Value");

    let mut value = String::new();

    io::stdin().read_line(&mut value).expect("Some error happens");

    println!("Now Please Enter a Unit: (F for Fahrenheit) and (C for Celsius)");

    let mut unit = String::new();

    io::stdin().read_line(&mut unit).expect("Some error happens");

    let value: f32 = value.trim().parse().expect("Some error happens");

    if unit.trim() == "F" {

        let calculated = (value - 32.0) * 5.9;
        println!("You choosed {} of °{}", value, unit);
        println!("And Result is {} of °C", calculated);

    } else if unit.trim() == "C" {

        let calculated = (value * 9.5) + 32.0;
        println!("You choosed {} of °{}", value, unit);
        println!("And Result is {} of °F", calculated);

    } else {
        print!("We don't know about this unit\n")
    }

    println!("=====================================================================");
}
