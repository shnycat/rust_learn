const PI: f64 = 3.1415926;
const PHI: f32 = 1.6;
const MAX: i32 = 32768;

// constant should be all upper case
// constant must be provided a type

fn main() {
    println!("{}", PI);
    println!("{}", PHI);

    let mut x = 0;
    for _ in 1..MAX {
        x += 1;
    }

    println!("{}", x);
}
