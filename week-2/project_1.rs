fn main() {
    // Given values
    let principal = 520_000_000.0; 
    let rate = 10.0;               
    let years = 5.0;               

    let amount = principal * (1.0 + (rate / 100.0)).powf(years);

    let compound_interest = amount - principal;

    println!("Total Amount after 5 years: ₦{:.2}", amount);
    println!("Compound Interest: ₦{:.2}", compound_interest);