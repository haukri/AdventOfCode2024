/// Entry point for the program
///
/// Read input from file, calculates the total distance and prints the result.
fn main() {
    let result = calculate_total_distance("src/input.txt");

    match result {
        Ok(total_distance) => {
            println!("Total distance: {total_distance}");
        }
        Err(e) => {
            eprintln!("Error occured {e}");
        }
    }
}

/// Processes the contents of the input file
///
/// Splits each line into two integers and collects them into separate vectors
///
/// # Arguments
///
/// * `contents` - A string containing the file contents.
///
/// # Returns
///
/// * `Ok(i32)` - The total distance if the file and its contents are valid.
/// * `Err` - An error if file reading or parsing fails.
fn process_lines(contents: String) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let vec1 = contents
        .lines()
        .filter_map(|line| line.split_whitespace().nth(0)?.parse::<i32>().ok())
        .collect(); // collect the first numbers of each line into vec1

    let vec2 = contents
        .lines()
        .filter_map(|line| line.split_whitespace().nth(1)?.parse::<i32>().ok())
        .collect(); // collect the second number of each line into vec2

    Ok((vec1, vec2))
}

fn calculate_total_distance(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file_path)?;

    let (mut vec1, mut vec2) = process_lines(contents)?;

    vec1.sort();
    vec2.sort();

    let total_distance: i32 = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Ok(total_distance)
}

#[test]
/// Tests the `calculate_total_distance` function using a temporary file.
///
/// Creates a temporary input file, writes test data to it, and verifies that
/// the function correctly calculates the total distance. Cleans up the file afterward.
fn test_total_distance() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::{self, File};
    use std::io::Write;

    let file_path = "test_input.txt";
    let mut temp_file = File::create(file_path)?;

    let test_data = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
    temp_file.write_all(test_data.as_bytes())?;

    let result = calculate_total_distance(file_path);

    match result {
        Ok(total_distance) => {
            assert_eq!(total_distance, 11, "The total distance should be 11.");
        }
        Err(e) => {
            panic!("Test failed with error: {e}");
        }
    }

    fs::remove_file(file_path)?;

    Ok(())
}
