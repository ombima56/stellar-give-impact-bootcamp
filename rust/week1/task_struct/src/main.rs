enum Flavor {
    Lemon,
    Mango,
    Orange,
    Strawberry,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    let flavor_name = match drink.flavor {
        Flavor::Lemon => "Lemon",
        Flavor::Mango => "Mango",
        Flavor::Orange => "Orange",
        Flavor::Strawberry => "Strawberry",
    };
    println!("Flavor: {}, Fluid oz: {}", flavor_name, drink.fluid_oz);
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Mango,
        fluid_oz: 12.0,
    };

    print_drink(drink);
}
