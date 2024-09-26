fn main() {
    println!("=== variable declaration ===");
    // semicolon is mandatory except for the last statement
    let n = 75;
    // variables are unmutable by default
    // cannot change the value after declaration
    //n = 69; // (this'll throw an error)

    println!("n = {}", n);

    let mut x = 42; // add 'mut' to make it mutable
    println!("x = {}", x);
    x = 21; // won't throw error
    println!("x is now = {}", x);

    println!("\n=== data types ===");
    let name: &str = "Mob Shigeo"; // string
    let initial: char = 'M'; // ascii characters
    let age: i32 = 21; // 32 bit integer, i64 = 64 bit
    let height: f32 = 177.3; // 32 bit floating point, f64
    let alive: bool = false;

    println!("His name is {}", name);
    println!("Initial: {}", initial);
    println!("He is {} years old", age);
    println!("He is {} centimeters tall", height);
    println!("Is he alive: {}", alive);

    println!("\n=== arithmetics ===");
    let a = 22.; // float
    let b = 7.;

    println!("a = {}, b = {}:", a, b);
    println!("a + b = {}", a + b); // addition
    println!("a - b = {}", a - b); // subtraction
    println!("a * b = {}", a * b); // multiplication
    println!("a / b = {}", a / b); // division
    println!("a % b = {}", a % b); // remainder or modulo

    // can't do arithmetic on different data type
    let c = 8; // integer, error:
               //println!("b * c = {}", b + c); // error

    println!("c = {}", c);
}
