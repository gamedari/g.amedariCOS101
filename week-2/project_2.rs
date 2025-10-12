fn main() {
    let sales = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

    let sum:f64 = sales.iter().sum();

    let average = sum / sales.len() as f64;

    println!("Total Sales Amount: ₦{:.2}", sum);
    println!("Average Sales Amount: ₦{:.2}", average);
}