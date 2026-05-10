enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Yellow => println!("Yellow"),
    }
}

fn main() {
    print_color(Color::Green);
}
