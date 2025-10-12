fn main() {
    // Given values
    let principal = 510_000.0; 
    let rate = 5.0;           
    let years = 3.0;         

    let amount = principal * (1.0 - (rate / 100.0)).powf(years);

    println!("The value of the TV after 3 years is ₦{:.2}", amount);
}