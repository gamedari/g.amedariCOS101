use std::fs::File;
use std::io::Write;

fn main() {
    let students : Vec<(&str, &str, &str, u32)> = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];    

    let mut file = File::create("data.csv").expect("failed");
    file.write_all("PAU SIMS\n".as_bytes()).expect("write failed");
    file.write_all("Student Name,Matric Number,Department,Level\n".as_bytes()).expect("write failed");
    for (name, matric, department, level) in &students  {
        file.write_all(name.as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");        
        file.write_all(matric.as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");        
        file.write_all(department.as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");        
        file.write_all(level.to_string().as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");        

    }
}

