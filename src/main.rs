use std::fs;

fn main() {
    //loads from file
    let data = fs::read_to_string("data.txt").expect("Unable to read file");

    //loop
    let number = 999999; 
    for num in 0..number { // change it to get range
        println!("{}", data);
    }
}