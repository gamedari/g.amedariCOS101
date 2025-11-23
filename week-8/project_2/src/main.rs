fn main() {
    let candidates = vec![
        ("Alice", 3),
        ("Bola", 7),
        ("Chinedu", 5),
        ("Diana", 9),
        ("Emeka", 4),
    ];

    // Assume the first candidate has the highest experience
    let mut highest = candidates[0];

    // Loop through the vector to find the candidate with the highest experience
    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = *candidate;
        }
    }

    println!(
        "The candidate with the highest years of experience is {} with {} years.", highest.0, highest.1
    );
}
