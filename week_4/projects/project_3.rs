std::io;

fn main() {
    let mut experience = String::new();
    let mut age_input = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    println!("Enter the employee's age:");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    let incentive = if experience == "yes" {
        if age >= 40 {
            1_560_000
        } else if age >= 30 && age <= 39 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            1_000_000 // Optional: handle ages between 28–29 if needed
        }
    } else {
        100_000
    };

    println!("Annual incentive: ₦{}", incentive)
}