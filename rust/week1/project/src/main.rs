use std::collections::HashMap;
use std::io;

struct Bills {
    inner: HashMap<String, f64>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, name: String, amount: f64) -> bool {
        if self.inner.contains_key(&name) {
            return false;
        }
        self.inner.insert(name, amount);
        true
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        if let Some(bill) = self.inner.get_mut(name) {
            *bill = amount;
            true
        } else {
            false
        }
    }

    fn view(&self) {
        if self.inner.is_empty() {
            println!("No bills found.");
        } else {
            let mut bills: Vec<(&String, &f64)> = self.inner.iter().collect();
            bills.sort_by_key(|(name, _)| name.to_lowercase());
            for (name, amount) in bills {
                println!("  {} - ${:.2}", name, amount);
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    if !prompt.is_empty() {
        println!("{}", prompt);
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_amount(prompt: &str) -> Option<f64> {
    loop {
        let input = get_input(prompt);
        match input.parse::<f64>() {
            Ok(amount) if amount > 0.0 => return Some(amount),
            Ok(_) => println!("Amount must be greater than zero. Try again:"),
            Err(_) => println!("\"{}\" is not a valid number. Try again:", input),
        }
    }
}

fn add_bill(bills: &mut Bills) {
    let name = get_input("Enter bill name:");
    if name.is_empty() {
        println!("Name cannot be empty.");
        return;
    }
    let amount = get_amount("Enter amount:");
    if bills.add(name.clone(), amount.unwrap()) {
        println!("Bill added.");
    } else {
        println!("A bill named \"{}\" already exists.", name);
    }
}

fn remove_bill(bills: &mut Bills) {
    bills.view();
    let name = get_input("Enter bill name to remove:");
    if bills.remove(&name) {
        println!("Bill removed.");
    } else {
        println!("Bill \"{}\" not found.", name);
    }
}

fn edit_bill(bills: &mut Bills) {
    bills.view();
    let name = get_input("Enter bill name to edit:");
    let amount = get_amount("Enter new amount:");
    if bills.update(&name, amount.unwrap()) {
        println!("Bill updated.");
    } else {
        println!("Bill \"{}\" not found.", name);
    }
}

fn main_menu() {
    println!("\n--- Bill Manager ---");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Edit bill");
    println!("5. Quit");
    println!("Enter choice:");
}

fn main() {
    let mut bills = Bills::new();

    loop {
        main_menu();
        let choice = get_input("");

        match choice.as_str() {
            "1" => add_bill(&mut bills),
            "2" => bills.view(),
            "3" => remove_bill(&mut bills),
            "4" => edit_bill(&mut bills),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}
