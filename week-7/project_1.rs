use std::io;

fn trapezium_area(height:f64, base1:f64, base2:f64)->f64 {
   height/2.0 * (base1 + base2)
}

fn rhombus_area(diagonal1:f64, diagonal2:f64,)->f64 {
   0.5 * diagonal1 * diagonal2
}

fn parallelogram_area(base:f64, height:f64)->f64 {
   base * height
}

fn cube_surface_area(side:f64)->f64 {
   6.0 * side * side
}

fn cylinder_volume(radius:f64, height:f64)->f64 {
   std::f64::consts::PI * radius * radius * height 
}

fn main() {
   println!("The Shape Calculator");
 
   println!("Shape\tCalculation");
   println!("Trapezium\tArea");
   println!("Rhombus\tArea");
   println!("Parallelogram\tArea");
   println!("Cube\tSurface Area");
   println!("Cylinder\tVolume");

   println!("Choose a shape from the menu");

   let mut choice = String::new();
   io::stdin().read_line(&mut choice).unwrap();
   let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "trapezium" => {
            let (height, base1, base2) = read_three("height", "base1", "base2");
            println!("Area = {}cm²", trapezium_area(height, base1, base2));
        }
        "rhombus" => {
            let (d1, d2) = read_two("diagonal1", "diagonal2");
            println!("Area = {}cm²", rhombus_area(d1, d2));
        }
        "parallelogram" => {
            let (base, height) = read_two("base", "height");
            println!("Area = {}cm²", parallelogram_area(base, height));
        }
        "cube" => {
            let side = read_one("side");
            println!("Surface area = {}cm²", cube_surface_area(side));
        }
        "cylinder" => {
            let (radius, height) = read_two("radius", "height");
            println!("Volume = {}cm³", cylinder_volume(radius, height));
        }
        _ => println!("Invalid choice."),
    }
}

fn read_one(label: &str) -> f64 {
    println!("Enter {}:", label);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_two(label1: &str, label2: &str) -> (f64, f64) {
    (read_one(label1), read_one(label2))
}

fn read_three(label1: &str, label2: &str, label3: &str) -> (f64, f64, f64) {
    (read_one(label1), read_one(label2), read_one(label3))
}