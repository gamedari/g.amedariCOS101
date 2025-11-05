use std::io;

fn main() {
    // Display menu
    println!("========================================");
    println!("    COMPUTER STORE INVENTORY SYSTEM");
    println!("========================================");
    println!("\nAvailable Items:");
    println!("--------------------------------------------------");
    println!("Code\tItem\t\tPrice (₦)");
    println!("--------------------------------------------------");
    println!("L\tLaptop\t\t550,000");
    println!("M\tMonitor\t\t120,000");
    println!("K\tKeyboard\t15,000");
    println!("H\tHeadset\t\t25,000");
    println!("--------------------------------------------------\n");

    // Get item code from user
    let mut item_code = String::new();
    println!("Enter item code (L/M/K/H): ");
    io::stdin()
        .read_line(&mut item_code)
        .expect("Failed to read input");
    
    let item_code = item_code.trim().to_uppercase();

    // Get quantity from user
    let mut quantity_input = String::new();
    println!("Enter quantity: ");
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");
    
    let quantity: u32 = match quantity_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity entered!");
            return;
        }
    };

    // Determine item and price
    let (item_name, unit_price) = match item_code.as_str() {
        "L" => ("Laptop", 550000.0),
        "M" => ("Monitor", 120000.0),
        "K" => ("Keyboard", 15000.0),
        "H" => ("Headset", 25000.0),
        _ => {
            println!("Invalid item code!");
            return;
        }
    };

    // Calculate total cost
    let total_cost = unit_price * quantity as f64;
    
    println!("\n========================================");
    println!("           ORDER SUMMARY");
    println!("========================================");
    println!("Item: {}", item_name);
    println!("Unit Price: ₦{:.2}", unit_price);
    println!("Quantity: {}", quantity);
    println!("--------------------------------------------------");
    println!("Subtotal: ₦{:.2}", total_cost);

    // Apply discount if total exceeds ₦500,000
    if total_cost > 500000.0 {
        let discount = total_cost * 0.07;
        let final_cost = total_cost - discount;
        
        println!("Discount (7%): -₦{:.2}", discount);
        println!("--------------------------------------------------");
        println!("FINAL TOTAL: ₦{:.2}", final_cost);
    } else {
        println!("--------------------------------------------------");
        println!("FINAL TOTAL: ₦{:.2}", total_cost);
    }
    println!("========================================\n");
}