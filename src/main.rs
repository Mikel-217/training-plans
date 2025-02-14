use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{create_dir, read_dir, File, ReadDir};
use std::path::Path;


#[derive(Serialize, Deserialize)]
struct Exersice {
    Name: String,
    Descriptio: String,
    Duration: i16
}

#[derive(Serialize, Deserialize)]
struct Plan {
    Name: String,
    Exercises: Vec<Exersice>,
    Duration: i16
}

fn main() {
    startup();
    println!("Hello, which plan may it be for you today? Select one :)");
    
}

fn startup() {
    let parentpath = r"C:\trainingsplan\";
    let exersicepath = r"C:\trainingsplan\exersice";
    let planpath = r"C:\trainingsplan\plans";

    if Path::new(parentpath).exists() && Path::new(exersicepath).exists() && Path::new(planpath).exists() {
        get_plans();
    } else {
        create_dir(parentpath).expect("Cannot Create Directory");
        create_dir(exersicepath).expect("Cannot Create Directory");
        create_dir(planpath).expect("Cannot Create Directory");
        create_plans();
    }

}


fn get_plans() -> Vec<String> {
    let path = r"C:\trainingsplan\plans";
    let mut plans: Vec<String> = Vec::new();
    
    for result in read_dir(path) {
        let names = result.filter_map(|entry| {entry.ok().and_then(|e| e.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s))))}).collect();
        plans.push(names);
    }
    plans
}

fn create_plans() {
    println!("I work :D");
}


