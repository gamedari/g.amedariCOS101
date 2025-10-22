use std::io;

fn main() {
    let mut experience = String::new();
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    let mut age_str = String::new();
    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut age_str).expect("Failed to read input");
    let age: u32 = age_str.trim().parse().expect("Invalid age entered");


    if experience == "yes" {
        if age >= 40 {
            println!("The incentive is ₦1,560,000 per year.");
        } else if age >= 30 && age < 40 {
            println!("The incentive is ₦1,480,000 per year.");
        } else if age < 28 {
            println!("The incentive is ₦1,300,000 per month.");
        } else {
            println!("No specific incentive category for this age group.");
        }
    } else if experience == "no" {
        println!("The incentive is ₦100,000 per year.");
    } else {
        println!("Invalid input for experience. Please enter 'yes' or 'no'.");
    }
}
