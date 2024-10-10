fn main() {
    // infinite loop with condition to break
    let mut i = 1;
    loop {
        println!("'loop' i = {}", i);
        i += 1;
        if i > 5 {
            break;
        }
    }

    // while loop equivalent of above loop
    i = 1;
    while i < 6 {
        println!("'while' i = {}", i);
        i += 1;
    }

    // for loop
    for n in 1..6 {
        println!("'for' n = {}", n);
    }

    let pet = vec!["Tom the cat", "Doge the dog", "Jerry the mouse"];
    for p in &pet {
        println!("&pet: {}", p);
    }

    for (idx, p) in pet.iter().enumerate() {
        println!("pet[{}]: {}", idx, p);
    }
}
