fn main() {
   
   // Vector of tuples: (name, years_of_experience)
    let candidates = vec![
        ("Ackerman", 4),
        ("Bulma", 7),
        ("Cristiano", 3),
        ("Diane", 9),
        ("Eren", 11),
    ];

    // Track the person with the highest experience
    let mut highest = ("", 0);

    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = *candidate;
        }
    }

    println!("The person with the highest programming experience is {} with {} years.",highest.0, highest.1);
}