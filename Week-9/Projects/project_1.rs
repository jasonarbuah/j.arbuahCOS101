 use std::io::Write;

 fn main() {

    let header = "Lager,Stout,Non-Alcholic\n";
    let row1 = "33 Export,Legend,Maltina\n";
    let row2 = "Desperados,Turbo King,Amstel Malta\n";
    let row3 = "Goldberg,Williams,Malta Gold\n";
    let row4 = "Gulder,,Fayrouz\n";
    let row5 = "Heineken,,\n";
    let row6 = "Star,,\n";

    let mut file = std::fs::File::create("categories_of_drinks.csv").expect("create failed");
    file.write_all("Welcome to Nigerian Breweries Plc\n".as_bytes()).expect("write failed");
    // Drink responsibly
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row1.as_bytes()).expect("write failed");
    file.write_all(row2.as_bytes()).expect("write failed");
    file.write_all(row3.as_bytes()).expect("write failed");
    file.write_all(row4.as_bytes()).expect("write failed");
    file.write_all(row5.as_bytes()).expect("write failed");
    file.write_all(row6.as_bytes()).expect("write failed");
    println!("\nCategories of drinks written to file.")
}