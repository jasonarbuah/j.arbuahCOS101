 // define total cost of each laptop 
   struct Laptops {
        quantity:u32,
        price:u32,
        brand: String
   }
   // logic to calculate the toatal cost of each laptop
   impl Laptops {
       fn total_cost(&self)->u32 {
       //use the . operator to fetch the value of a field via the self keyword
       self.quantity * self.price
       }
   }
 
   fn main() {
      // instantiate the structure
      let lappy1 = Laptops {
          quantity: 3,
          price: 650_000,
          brand:String::from("HP")
        };
      let lappy2 = Laptops {
          quantity: 3,
          price: 755_000,
          brand:String::from("IBM"),
        };
      let lappy3 = Laptops {
          quantity: 3,
          price: 550_000,
          brand:String::from("Toshiba")
        };
      let lappy4 = Laptops {
          quantity: 3,
          price: 850_000,
          brand:String::from("Dell")
        };
    //pass lappy1, lappy2, lappy3 and lappy4 to display()
    display(&lappy1);
    display(&lappy2);
    display(&lappy3);
    display(&lappy4);
 
 // fetch values of specific structure fields using the
 // operator and print it to the console
let total_cost = lappy1.total_cost() + lappy2.total_cost() + lappy3.total_cost() + lappy4.total_cost();
println!("\nTotal cost is: N{}", total_cost)
}
fn display(lappy:&Laptops) {
    println!("Quantity is :{} Total cost is N{} Brand is {}",
        lappy.quantity, lappy.total_cost(), lappy.brand);
}
 