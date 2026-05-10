fn main() {
    first_name("Hillary");
    last_name("Ombima");
}

// Use a function to display your first name
// Use a function to display your last name
// Use the println macro to display messages to the terminal

fn first_name(name: &str) {
    println!("First name is {}", name);
}

fn last_name(name: &str) {
    println!("Last name is {}", name);
}