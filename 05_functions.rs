fn main() {
    say_hi_to("Mob");
    say_hi_to("Mutsumi-chan");

    for i in 1..11 {
        if is_even(i) {
            println!("{} is even", i);
        } else {
            println!("{} is odd", i);
        }
    }

    println!("43 - 27 = {}", sub(43, 27));
    println!("43 + 27 = {}", add(43, 27));
}

// no return function
fn say_hi_to(name: &str) {
    println!("Hello there, {}!", name);
}

// function with return type of boolean
fn is_even(n: u32) -> bool {
    return n % 2 == 0;
}

// function with return type of 32 bit integer
fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

