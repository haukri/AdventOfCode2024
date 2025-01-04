use anyhow::{Context, Result};
use regex::Regex;
use std::time::Instant;

/// Entry point for the program
///
/// This program finds all multiplications in the input, performs the multiplication and sum the results up
fn main() {
    let start = Instant::now();

    let result = get_multiplication_result("src/input.txt");

    match result {
        Ok(multiplication_result) => {
            println!("Sum of multiplications: {multiplication_result}");
        }
        Err(e) => {
            eprintln!("Error occured: {e:?}");
        }
    }

    let duration = start.elapsed();
    println!("Execution time: {:.2?}", duration);

}

fn get_multiplication_result(file_path : &str) -> Result<i32> {
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {file_path}"))?;

    let re = Regex::new(r"mul\((?<num1>\d+),(?<num2>\d+)\)") // Regex that matches mul(1,2) where the two numbers are named captures num1 and num2
        .context("Failed create regex expression")?;

    let multiplication_result = re.captures_iter(&content)
        .try_fold(0, |acc, x| {
            let result = parse_and_multiply(&x["num1"], &x["num2"])?;
            Ok::<i32, anyhow::Error>(acc + result)
        })?;

    Ok(multiplication_result)
}

fn parse_and_multiply(num1: &str, num2: &str) -> Result<i32> {
    let num1 = num1.parse::<i32>().context("Invalid number for num1")?;
    let num2 = num2.parse::<i32>().context("Invalid number for num2")?;

    Ok(num1 * num2)
}