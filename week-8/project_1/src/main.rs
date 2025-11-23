use std::io;

fn main() {
    // Create vectors to store the job data
    let job_titles = vec![
        "Intern", "Paralegal", "Placement", "Administrator", "Research Assistant",
        "Junior Associate", "Classroom Teacher", "Senior Administrator", "PhD Candidate",
        "Associate", "Snr Teacher", "Office Manager", "Post-Doc Researcher", 
        "Senior Associate 1-2", "Leading Teacher", "Director", "Senior Lecturer",
        "Senior Associate 3-4", "Deputy Principal", "CEO", "Dean", "Partner", "Principal"
    ];
    
    let levels = vec![
        "APS 1-2", "APS 1-2", "APS 1-2", "APS 3-5", "APS 3-5",
        "APS 3-5", "APS 3-5", "APS 5-8", "APS 5-8", "APS 5-8",
        "APS 5-8", "EL1 8-10", "EL1 8-10", "EL1 8-10", "EL1 8-10",
        "EL2 10-13", "EL2 10-13", "EL2 10-13", "EL2 10-13", "SES",
        "SES", "SES", "SES"
    ];
    
    let departments = vec![
        "Office Administrator", "Lawyer", "Teacher", "Office Administrator", "Academic",
        "Lawyer", "Teacher", "Office Administrator", "Academic", "Lawyer",
        "Teacher", "Office Administrator", "Academic", "Lawyer", "Teacher",
        "Office Administrator", "Academic", "Lawyer", "Teacher", "Office Administrator",
        "Academic", "Lawyer", "Teacher"
    ];
    
    let min_experience = vec![1, 1, 1, 3, 3, 3, 3, 5, 5, 5, 5, 8, 8, 8, 8, 10, 10, 10, 10, 13, 13, 13, 13];
    
    let max_experience = vec![2, 2, 2, 5, 5, 5, 5, 8, 8, 8, 8, 10, 10, 10, 10, 13, 13, 13, 13, 50, 50, 50, 50];

    println!("Enter the staff member's job title (e.g., Associate):");
    let mut input_title = String::new();
    io::stdin().read_line(&mut input_title).expect("Failed to read line");
    let mut clean_title = input_title.trim();

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
    let mut department_title = format!("Department not found");

    // Iterate through the vectors
    for i in 0..job_titles.len() {
        if job_titles[i] == clean_title {
            if experience_years >= min_experience[i] && experience_years <= max_experience[i] {
                result_message = format!("The staff member holds position: {} ({} to {} years experience required).", 
                    levels[i], min_experience[i], max_experience[i]);
                department_title = format!("Department: {}", departments[i]);
                break; // Found the correct, validated level
            } else {
                result_message = format!("Found title '{}', but {} years of experience does not match the required range of {} to {} years for this level ({}).", 
                    clean_title, experience_years, min_experience[i], max_experience[i], levels[i]);
                department_title = format!("Department: {}", departments[i]);
                break; // Found the title but experience is wrong
            }
        }
    }


    println!("\n--- Result ---");
    println!("{}", department_title);
    println!("Position: {}", clean_title);
    println!("{}", result_message);
}