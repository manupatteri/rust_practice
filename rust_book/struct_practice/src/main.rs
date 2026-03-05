use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let my_table = Table { id : 1, name: String::from("Manoj") };
    let file_path = "table.txt";

    let json = serde_json::to_string_pretty(&my_table)?;
    fs::write(file_path, json)?;

    println!("Wrote json at {}", file_path);
    Ok(())

}
#[derive(Serialize, Deserialize, Debug)]
struct Table {
    id: u64,
    name: String
}
