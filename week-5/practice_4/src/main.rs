fn main() {
    let fullname = "Greigns Ameri";
    let department = "Data Science";
    let uni = "PAU";

    let mut school = "School of Science".to_string();
    school.push_str(" and Tech");

    println!("My name is {}", fullname);
    println!("The lenght my full name is {}", fullname.len());
    println!("I am a student of {} Department", department);
}
