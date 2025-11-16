use std::io;

fn trapezium() {
    let mut h = String::new();
    let mut b1 = String::new();
    let mut b2 = String::new();

    println!("Enter height:");
    io::stdin().read_line(&mut h).unwrap();

    println!("Enter base1:");
    io::stdin().read_line(&mut b1).unwrap();

    println!("Enter base2:");
    io::stdin().read_line(&mut b2).unwrap();

    let h: f64 = h.trim().parse().unwrap();
    let b1: f64 = b1.trim().parse().unwrap();
    let b2: f64 = b2.trim().parse().unwrap();

    let area = h / 2.0 * (b1 + b2);
    println!("Area of Trapezium = {}cm^2", area);
}

fn rhombus() {
    let mut d1 = String::new();
    let mut d2 = String::new();

    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut d1).unwrap();

    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut d2).unwrap();

    let d1: f64 = d1.trim().parse().unwrap();
    let d2: f64 = d2.trim().parse().unwrap();

    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}cm^2", area);
}

fn parallelogram() {
    let mut base = String::new();
    let mut alt = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut base).unwrap();

    println!("Enter altitude:");
    io::stdin().read_line(&mut alt).unwrap();

    let base: f64 = base.trim().parse().unwrap();
    let alt: f64 = alt.trim().parse().unwrap();

    let area = base * alt;
    println!("Area of Parallelogram = {}cm^2", area);
}

fn cube() {
    let mut s = String::new();

    println!("Enter length of side:");
    io::stdin().read_line(&mut s).unwrap();

    let s: f64 = s.trim().parse().unwrap();

    let area = 6.0 * s * s;
    println!("Area of Cube = {}cm^2", area);
}

fn cylinder() {
    let mut r = String::new();
    let mut h = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut r).unwrap();

    println!("Enter height:");
    io::stdin().read_line(&mut h).unwrap();

    let r: f64 = r.trim().parse().unwrap();
    let h: f64 = h.trim().parse().unwrap();

    let pi = 3.142; 

    let volume = pi * r * r * h;
    println!("Volume of Cylinder = {}cm^3", volume);
}


fn main() {
    println!("Select the shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap();

    match choice {
        1 => trapezium(),
        2 => rhombus(),
        3 => parallelogram(),
        4 => cube(),
        5 => cylinder(),
        _ => println!("Invalid choice! Select between 1 and 5."),
    }
}
