use std::io;

fn main()
{

    println!("Is the employee experienced? (yes/no):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let _experience = experience.trim().to_lowercase();

    if _experience == "no"
    {
       println!("Annual incentive: N100,000");
       return
    }

    println!("\nEnter the employee's age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:u8 = age.trim().parse().expect("Input not an integer");
   
    if age >= 40
    {
       println!("Annual incentive is N1,560,000");
    }
    else if age >= 30 && age <= 39
    {
       println!("Annual incentive is N1,480,000");
    }
    else if age <= 28
    {
       println!("Annual incentive is N1,300,000");
    }
    else
     {
       println!("Annual incentive is N100,000");
     } 
}