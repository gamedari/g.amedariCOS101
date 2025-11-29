use std::fs::File;
use std::io::Write;

fn main() {
    let drinks: Vec<(&str, Vec<&str>)> = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"])
    ];

    let mut file = File::create("drinks.txt").expect("Failed to create file");
    for (category, drinks_list) in &drinks {
        file.write_all("\n".as_bytes()).expect("write failed");
        file.write_all(category.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");

        for item in drinks_list{
            file.write_all(item.as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");
        }
    }
}