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

fn process_lines(contents : String) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>>{

    let vec1 = contents.lines()
        .filter_map(|line| line.split_whitespace().nth(0)?.parse::<i32>().ok())
        .collect();

    let vec2 = contents.lines()
        .filter_map(|line| line.split_whitespace().nth(1)?.parse::<i32>().ok())
        .collect();

    Ok((vec1, vec2))
}

fn calculate_total_distance(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file_path)?;

    let (mut vec1,mut vec2) = process_lines(contents)?;

    vec1.sort();
    vec2.sort();

    let total_distance: i32 = vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Ok(total_distance)
}

#[test]
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