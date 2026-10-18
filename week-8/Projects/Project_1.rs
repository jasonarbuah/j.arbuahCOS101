 use std::io;

 fn main() {

    let levels = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];

    let office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let academic = vec!["-","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let teacher = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];

    let mut profession = String::new();
    let mut role = String::new();
    let mut years_str = String::new();

    println!("Enter profession (Office Admin, Academic, Lawyer, Teacher): ");
    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession:String = profession.trim().parse().expect("Invalid input");

    println!("Enter role: ");
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role:String = role.trim().parse().expect("Invalid input");

    println!("Enter years of experience: ");
    io::stdin().read_line(&mut years_str).expect("Failed to read input");
    let years: u16 = years_str.trim().parse().expect("Invalid input");

    let prof_roles = if profession == "Office Admin" {
        &office_admin
    } else if profession == "Academic" {
        &academic
    } else if profession == "Lawyer" {
        &lawyer
    } else if profession == "Teacher" {
        &teacher
    } else {
        println!("Unknown profession!");
        return;
    };

    let mut index: usize = 0;
    let mut found = false;
    for i in 0..prof_roles.len() {
        if prof_roles[i] == role {
             index = i;
             found = true;
        }
    }

    if !found {
        println!("Role '{}' not found in {}", role, profession);
        return;
    }

    let level = levels[index];
    let valid = match level {
        "APS 1-2" => years <= 2,
        "APS 3-5" => years >= 3 && years <=5,
        "APS 5-8" => years >= 5 && years <=8,
        "EL1 8-10" => years >= 8 && years <= 10,
        "EL2 10-13" => years >= 10 && years <= 13,
        "SES" => years >= 13,
        _ => false,
    };

    if valid {
       println!("{} ({}, {} years) = {}", role, profession, years, level);
    } else {
       println!("{} ({}, {} years) does not match expected range for {}", role, profession, years, level);
    }
}
