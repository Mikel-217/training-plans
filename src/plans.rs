use std::{fs::{read_dir, File}, io::{self, Read}, ptr::{null, read}};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct Plan {
    Name: String,
    Exercises: Vec<Exersice>,
    Duration: i16
}

#[derive(Deserialize, Serialize)]
struct Exersice {
    Name: String,
    Descriptio: String,
    Duration: i16
}


pub fn create_plans() {
    
    let exersices = get_exersice();
    if exersices.is_empty() {
        println!("No Exersices found :(");
    }

    let mut name = String::new();
    let ids = 0;
    
    println!("Please Enter the name of your plan:");
    io::stdin().read_line(&mut name).expect("Not a valid input");
    
    for values in exersices {
        ids + 1;
        println!("{ids} {values}");
    }

}

fn get_exersice() -> Vec<String> {
    let path = r"C:\trainingsplan\exersice";

    let mut exersice: Vec<String> = Vec::new();

    for result in read_dir(path) {
        let names: String = result.filter_map(|entry| {entry.ok().and_then(|e| e.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s))))}).collect();
        exersice.push(names);
    }
    exersice
}

fn createfile(filename: String, plan: Plan) {
    let mut path = String::from(r"C:\trainingsplan\plans");
    path.push_str(&filename);
    path.push_str(".json");
    println!("{path}");
    
    let mut file = File::create(path).expect("Error while creating file");
    // let writesr: Plan = serde_json::to_writer(, &plan).expect("Error while writing to file");


}

