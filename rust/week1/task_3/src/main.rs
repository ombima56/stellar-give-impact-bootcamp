fn main() {
    // Change this value to test all three branches: any int works
    let number = 4;

    if number > 5 {
        println!("greater than 5");
    } else if number < 5 {
        println!("less than 5");
    } else {
        // Only reachable when number == 5 exactly
        println!("=5");
    }
}
