use anyhow::{Context, Result};

/// Entry point for the program
///
/// This program counts the number of safe reports in the puzzle input
fn main() {
    let result = count_safe_reports("src/input.txt");

    match result {
        Ok(safe_report_count) => {
            println!("Number of safe reports: {safe_report_count}");
        }
        Err(e) => {
            eprintln!("Error occured: {e:?}");
        }
    }
}

fn count_safe_reports(file_path : &str) -> Result<i32> {
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {file_path}"))?;

    let report_result: Result<Vec<bool>> = content
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            report_is_safe(line)
                .with_context(|| format!("Error in line {} : {} ", line_number + 1, line))
        })
        .collect();

    match report_result {
        Ok(results) => {
            Ok(results.iter().filter(|&&is_safe| is_safe).count().try_into()?) // Check if all reports are safe
        }
        Err(e) => Err(e), // Propagate the error
    }
}

fn report_is_safe(report: &str) -> Result<bool> {
    let report: Vec<i32> = report
        .split_whitespace()
        .map(|level| level.parse::<i32>().with_context(|| format!("Invalid number: {level}")))
        .collect::<Result<Vec<i32>>>()?;

    let differences: Vec<i32> = report
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();

    let is_positive = differences.iter().all(|&x| x > 0 && x < 4);
    let is_negative = differences.iter().all(|&x| x < 0 && x > -4);

    Ok(is_positive || is_negative)
}

#[test]
/// Tests the `are_reports_safe` function using a temporary file.
///
/// Creates a temporary input file, writes test data to it, and verifies that
/// the function correctly calculates the total distance. Cleans up the file afterward.
fn test_are_reports_safe() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::{self, File};
    use std::io::Write;

    let file_path = "test_input.txt";
    let mut temp_file = File::create(file_path)?;

    let test_data = "7 6 4 2 1\n1 2 7 8 9\n9 7d 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    temp_file.write_all(test_data.as_bytes())?;

    let result = count_safe_reports(file_path);

    match result {
        Ok(safe_report_count) => {
            assert_eq!(safe_report_count, 2, "The safe report count should be 2");
        }
        Err(e) => {
            panic!("Test failed with error: {e}");
        }
    }

    fs::remove_file(file_path)?;

    Ok(())
}