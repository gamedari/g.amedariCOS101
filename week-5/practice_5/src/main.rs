fn main() {
    let fullname = "PAU";
    println!();
    println!("Name: {}", fullname);
    println!();
    println!("Before trim");
    println!("length is {}", fullname.len());
    println!();
    println!("After trim");
    println!("length is {}", fullname.trim().len());
}
