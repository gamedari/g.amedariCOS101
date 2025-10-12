fn main() {
    // Given values
    let principal:f64 = 510_000.0; 
    let rate:f64 = 5.0;           
    let years:f64 = 3.0;

    let amount = principal * (1.0 - (rate / 100.0)).powf(years);

    println!("The value of the TV after 3 years is ₦{:.2}", amount);
}