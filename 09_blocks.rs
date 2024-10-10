fn main() {
    let mut x = 9;
    let mut y = 7;

    println!("outer block: x{} y{}", x, y);

    {
        x = 20;
        let y = 999; // this 'y' is local to this block
        println!("y inner block is {}", y);
        println!("x inner block is {}", x);
    }

    println!("x is now {}", x);
    println!("y is now {}", y);
}
