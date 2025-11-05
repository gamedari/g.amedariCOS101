use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for a:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f64 = input1.trim().parse().expect("Invalid input for a");

    println!("Enter value for b:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f64 = input2.trim().parse().expect("Invalid input for b");

    println!("Enter value for c:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");    
    let c:f64 = input3.trim().parse().expect("Invalid input for c");

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots (discriminant is negative).");
    }
}
