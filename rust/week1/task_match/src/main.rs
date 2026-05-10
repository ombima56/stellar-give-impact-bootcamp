fn main() {
    // Part 1: match on a boolean
    let is_true = true;

    match is_true {
        true => println!("it's true"),
        false => println!("it's false"),
    }

    // Part 2: match on an integer
    let number = 2;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}
