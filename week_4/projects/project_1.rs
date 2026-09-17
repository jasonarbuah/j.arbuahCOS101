use std::io;
fn main() {
	println!("Enter your coefficient of x^2");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your coefficient of x");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

     println!("Enter your constant");
	let mut input3 = String::new();
	io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");
   
    // roots of quadratic equation
    let d = b * b - 4.0 * a * c;

    if d >= 0.0 {
    let root1 = (-b + d.sqrt() ) / (2.0 * a);
    let root2 = (-b - d.sqrt() ) / (2.0 * a);
   	println!("Two distinct roots: {} and {}", root1, root2);
    }

    else if d == 0.0 {
    	let root = -b / (2.0 * a);
   	    println!("Exactly one real root: {}", root);
    }
    else if d <= 0.0
    {
   	   println!("No real roots");
  }
}