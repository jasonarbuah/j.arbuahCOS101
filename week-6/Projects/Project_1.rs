use std::io;

fn main() {
   println!("Jay's Diner Menu:");
   println!("Food item\tPrice");
   println!("P: Poundo Yam / Edinkaiko Soup\tN3200");
   println!("F: Fried Rice & Chicken\tN3000");
   println!("A: Amala & Ewedu Soup\tN2500");
   println!("E: Eba & Egusi Soup\tN2000");
   println!("W: White Rice & Stew\tN2500");
 
   println!("\nEnter the letter for the food from the menu:");
   let mut choice = String::new();
   io::stdin().read_line(&mut choice).unwrap();
   let choice = choice.trim().to_uppercase();

   println!("Enter quantity:");
   let mut quantity = String::new();
   io::stdin().read_line(&mut quantity).unwrap();
   let quantity: i32 = quantity.trim().parse().unwrap();

   let price = match choice.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    let mut total = price * quantity;
    if total > 10_000 {
       total = (total as f64 * 0.95) as i32; // 5% discount
       println!("A 5% discount has been applied!");
    }

    println!("Total cost: N{}", total);
}