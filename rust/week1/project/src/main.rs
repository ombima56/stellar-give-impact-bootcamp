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

    fn add(&mut self, name: String, amount: f64) {
        self.inner.insert(name, amount);
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
            for (name, amount) in &self.inner {
                println!("  {} - ${:.2}", name, amount);
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_amount(prompt: &str) -> Option<f64> {
    let input = get_input(prompt);
    input.parse::<f64>().ok()
}

fn add_bill(bills: &mut Bills) {
    let name = get_input("Enter bill name:");
    if name.is_empty() {
        println!("Name cannot be empty.");
        return;
    }
    match get_amount("Enter amount:") {
        Some(amount) => {
            bills.add(name, amount);
            println!("Bill added.");
        }
        None => println!("Invalid amount."),
    }
}

fn remove_bill(bills: &mut Bills) {
    bills.view();
    let name = get_input("Enter bill name to remove:");
    if bills.remove(&name) {
        println!("Bill removed.");
    } else {
        println!("Bill not found.");
    }
}

fn edit_bill(bills: &mut Bills) {
    bills.view();
    let name = get_input("Enter bill name to edit:");
    match get_amount("Enter new amount:") {
        Some(amount) => {
            if bills.update(&name, amount) {
                println!("Bill updated.");
            } else {
                println!("Bill not found.");
            }
        }
        None => println!("Invalid amount."),
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
