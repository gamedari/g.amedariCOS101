use std::io;

fn add(a: i32, b: i32) {
    let sum = a + b;
    println!("Sum of A and B = {}", sum);
}

fn main() {
    let mut input1 = STring::new();
    println!("ENter input for paramter A:");
    io::stdin().read_line(&mut input1).expect("Failed to readinput");
    let a:i32 = input1.trim().parse().expect("Invalid input")

    let mut input2 = String::new();
    println!("ENter input for parameter B:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Invalid input");

    add(a, b)
}
