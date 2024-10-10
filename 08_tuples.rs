fn main() {
    let my_tuple = (69, 73.52, "attitude");

    println!("{}", my_tuple.0);
    println!("{}", my_tuple.1);
    println!("{}", my_tuple.2);

    let nums = (7, 9, (2, 3, 4));
    println!("{}", vol(nums.0, nums.1, (nums.2).2));
}

fn vol(a:i32, b:i32, c:i32) -> i32 {
    return a * b * c;
}
