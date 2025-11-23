use std::io;
use std::u32; // Import u32::MAX for open-ended range

// Storing data as (job_title, level_identifier, min_years, max_years)
static JOB_DATA_EXP: [(&str, &str, u32, u32); 23] = [
    ("Intern", "APS 1-2", 0, 2),
    ("Paralegal", "APS 1-2", 0, 2),
    ("Placement", "APS 1-2", 0, 2),
    ("Administrator", "APS 3-5", 3, 5),
    ("Research Assistant", "APS 3-5", 3, 5),
    ("Junior Associate", "APS 3-5", 3, 5),
    ("Classroom Teacher", "APS 3-5", 3, 5),
    ("Senior Administrator", "APS 5-8", 5, 8),
    ("PhD Candidate", "APS 5-8", 5, 8),
    ("Associate", "APS 5-8", 5, 8),
    ("Snr Teacher", "APS 5-8", 5, 8),
    ("Office Manager", "EL1 8-10", 8, 10),
    ("Post-Doc Researcher", "EL1 8-10", 8, 10),
    ("Senior Associate 1-2", "EL1 8-10", 8, 10),
    ("Leading Teacher", "EL1 8-10", 8, 10),
    ("Director", "EL2 10-13", 10, 13),
    ("Senior Lecturer", "EL2 10-13", 10, 13),
    ("Senior Associate 3-4", "EL2 10-13", 10, 13),
    ("Deputy Principal", "EL2 10-13", 10, 13),
    ("CEO", "SES", 13, u32::MAX),
    ("Dean", "SES", 13, u32::MAX),
    ("Partner", "SES", 13, u32::MAX),
    ("Principal", "SES", 13, u32::MAX),
];
fn main() {
    println!("Enter the staff member's job title (e.g., Associate):");
    let mut input_title = String::new();
    io::stdin().read_line(&mut input_title).expect("Failed to read line");
    let clean_title = input_title.trim().to_lowercase();

    println!("Enter the number of years of work experience (e.g., 6):");
    let mut input_exp = String::new();
    io::stdin().read_line(&mut input_exp).expect("Failed to read line");
    
    let experience_years: u32 = match input_exp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number entered for experience. Defaulting to 0 years.");
            0
        }
    };

    let mut result_message = format!("No matching level found for title '{}' with {} years of experience.", clean_title, experience_years);

    // Iterate through the static array of tuples
    for &(title, level, min_exp, max_exp) in JOB_DATA_EXP.iter() {
        if title.to_lowercase() == clean_title {
            if experience_years >= min_exp && experience_years <= max_exp {
                result_message = format!("The staff member holds position: {} ({} to {} years experience required).", level, min_exp, max_exp);
                break; // Found the correct, validated level
            } else {
                result_message = format!("Found title '{}', but {} years of experience does not match the required range of {} to {} years for this level ({}).", clean_title, experience_years, min_exp, max_exp, level);
                break; // Found the title but experience is wrong
            }
        }
    }

    println!("\n--- Result ---");
    println!("{}", result_message);
}
