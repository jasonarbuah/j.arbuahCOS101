 use std::io::Write;

 fn main() {
 // Vector of tuples: (student_name, matric_number, department, level)
    let _pau_smis = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemok", "MEE10202001", "Mechanical", 100),
    ];

    let header = "Student Name,Matric Number,Department,Level\n";
    let row1 = "Oluchi Mordi,ACC10211111,Accounting,300\n";
    let row2 = "Adams Aliyu,ECO10110101,Economics,100\n";
    let row3 = "Shania Bolade,CSC10328828,Computer,200\n";
    let row4 = "Adekunle Gold,EEE11020202,Electrical,200\n";
    let row5 = "Blanca Edemok,MEE10202001,Mechanical,100\n";

    let mut file = std::fs::File::create("pau_smis.csv").expect("create failed");
    file.write_all("PAU's Student Management Information System records\n".as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row1.as_bytes()).expect("write failed");
    file.write_all(row2.as_bytes()).expect("write failed");
    file.write_all(row3.as_bytes()).expect("write failed");
    file.write_all(row4.as_bytes()).expect("write failed");
    file.write_all(row5.as_bytes()).expect("write failed");

    use std::io::Read;

   let mut file = std::fs::File::open("pau_smis.csv").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   println!("{}", contents); 
}