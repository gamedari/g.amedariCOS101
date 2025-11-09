use std::io;
use std::process;

fn main() {
    println!("\nWelcome to Ame Bukka");
    println!("Menu\t\t\t\tPrice");
    println!("P = Poundo Yam/Edinkaiko Soup\tN3200");
    println!("F = Fried Rice & Chicken\tN3000");
    println!("A = Amala & Ewedu Soup\t\tN2500");
    println!("E = Eba & Egusi Soup\t\tN2000");
    println!("W = White Rice & Stew\t\tN2500");

    let mut input1 = String::new();
    println!("\nFood: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string"); 
    let food_init = input1.trim().to_uppercase();

    let mut input2 = String::new();
    println!("Quantity: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:i32 = input2.trim().parse().expect("Not a valid number");

    let mut price = 0;

    if food_init == "P" {
        price = 3200;
    }
    else if food_init == "F" {
        price = 3000;
    }
    else if food_init == "A" {
        price = 2500;
    }
    else if food_init == "E" {
        price = 2000;
    }
    else if food_init == "W" {
        price = 2500;
    }
    else {
        println!("Invalid letter entered");
        process::exit(0); // Exit with a non-zero status
    }

    let mut total = price * quantity;

    if total > 10000 {
        println!("\nYou received a 5% discount");
        println!("Amount before discount: {}", total);
        let discount =  0.95 * total as f64;
        total = discount as i32;
    }
    else {
        println!("\nNot eligible for the discount");
    }

    println!("==========================");
    println!("Total Amount to Pay: {}", total);
    println!("==========================");
    println!("Thank you for your order!");
}