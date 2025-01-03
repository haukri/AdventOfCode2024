use std::fs::File;
use std::io::prelude::*;

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occured: {e}")
    }
}

fn process_lines(contents : String) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>>{
    let mut vec1: Vec<i32>  = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut splitted = line.split_whitespace();
        if let (Some(num1), Some(num2)) = (splitted.next(), splitted.next()) {
            vec1.push(num1.parse::<i32>()?);
            vec2.push(num2.parse::<i32>()?);
        }
    }

    Ok((vec1, vec2))
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let (mut vec1,mut vec2) = process_lines(contents)?;

    vec1.sort();
    vec2.sort();

    let mut total_distance: i32 = 0;

    for (a, b) in vec1.iter().zip(vec2.iter()) {
        let distance = (a-b).abs();
        total_distance += distance;
    }

    println!("Total distance: {total_distance}");

    Ok(())
}