use std::io::Write;

fn main() {
    // Three separate datasets
    let _commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiyeye",
    ];

    let _ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let _zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Merge datasets by index

    let header = "Commissioners,Ministries,Zones\n";
    let row1 = "Aigbogun Alamba Daudu,Internal Affairs,South West\n";
    let row2 = "Murtala Afeez Bendu,Justice,North East\n";
    let row3 = "Okorocha Calistus Ogbonna,Defense,South South\n";
    let row4 = "Adewale Jimoh Akanbi,Power & Steel,South West\n";
    let row5 = "Osazuwa Faith Etiyeye,Petroleum,South East\n";

    let mut file = std::fs::File::create("efcc_merge.csv").expect("create failed");
    file.write_all("Files of convicted ministers from different geopolitical zones\n".as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row1.as_bytes()).expect("write failed");
    file.write_all(row2.as_bytes()).expect("write failed");
    file.write_all(row3.as_bytes()).expect("write failed");
    file.write_all(row4.as_bytes()).expect("write failed");
    file.write_all(row5.as_bytes()).expect("write failed");

    use std::io::Read;

   let mut file = std::fs::File::open("efcc_merge.csv").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   println!("{}", contents); 
}
