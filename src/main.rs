use std::result;

// First Program Hello World in Rust
fn main() {
    /*
    This is multi Line comment
     */
    let mut x = 10;
    println!("Hello, world!");
    println!("The value of x is: {}", x);
    x=20;
    println!("The value of x is: {}", x);

    let y = 30.353535353434;
    println!("The value of y is: {}", y);

    // adding two numbers
    let r = 5;
    let n = 10;
    let sum = r + n;
    println!("The sum of {} and {} is: {}", r, n, sum);

    // boolean values
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t is: {} and the value of f is: {}", t, f);
    println!("Not t is: {}", !t);
    println!("t and f is: {}", t & f);
    println!("t or f is: {}", t | f);
    println!("t xor f is: {}", t ^ f);

    // CHallenge to find the average
    let a: i32 = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;
    assert_eq!(average, 45.1);
    println!("Test passed!");

    //arrays
    let parkingLot = [[1, 2, 3], [4, 5, 6]];
    let number = parkingLot[0][1];
    println!("The number at parkingLot[0][1] is: {}", number);

    //tuples
    let mut stuff = (10, 3.14, 'x');
    stuff.0 = 3; // modifying the first element of the tuple
    let first_item = stuff.0;
    println!("The first item in the tuple is: {}", first_item);

    //Convert Temperature from Celcius to Fahrenheit
    let celcius_temp = 23.0;
    let fahrenheit_temp = celcius_to_fahrenheit(celcius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

//Function to convert Celcius to Fahrenheit
fn celcius_to_fahrenheit(temp: f64) -> f64 {
   temp * 1.8 + 32.0
   
}
