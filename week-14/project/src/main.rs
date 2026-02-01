use std::io;
use std::io::Read;

fn read_and_print_file(filename: &str) {
    let mut file = std::fs::File::open(filename)
        .expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    print!("{}", contents);
}

fn handle_role(role: &str) {
    match role {
        "employee" => read_and_print_file("staff_tb.sql"),
        "customer" => read_and_print_file("customers.sql"),
        "vendor" => read_and_print_file("dataplan.sql"),
        "project manager" => read_and_print_file("project.sql"),
        "administrator" => read_and_print_file("globacom_dbase.sql"),
        _ => println!("Invalid role entered"),
    }
}

fn main() {
    let mut role = String::new();

    println!("Enter your role: ");
    io::stdin()
        .read_line(&mut role)
        .expect("Failed to read input");

    let role = role.trim().to_lowercase();
    handle_role(&role);
}
