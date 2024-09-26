fn main() {
    let x = 5;
    let y = 6;

    println!("x = {}\ny = {}", x, y);

    if x > y {
        println!("x is greater than y")
    } else if x < y {
        println!("x is less than y");
    }

    if x == y {
        println!("x equals y");
    } else if x != y {
        println!("x not equals y");
    }

    if x >= 5 {
        println!("x is greater than or equal to 5");
    } else {
        println!("nevermind");
    }

    if y >= 64 {
        println!("y is greater than or equal to 5");
    } else {
        println!("nevermind");
    }
}
