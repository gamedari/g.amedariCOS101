fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan Atlantic University";
    let addr:&str = "Km 5 Lekki Epe Expressway, Ibeju Lekki, Lagos";
    println!("name: {}", name);
    println!("University: {}, \nAddress: {}", uni, addr);

    let department:&'static str = "Computer SCience";
    let school:&'static str = "School of SCience and Technology";
    println!("Dpeartment: {} \nSchool: {}", department, school);

}
