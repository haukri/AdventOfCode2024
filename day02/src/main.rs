/// Entry point for the program
///
/// Read input from file, calculates the total distance and prints the result.
fn main() {
    let result = are_reports_safe("src/input.txt");

    match result {
        Ok(reports_are_safe) => {
            println!("Reports is safe: {reports_are_safe}");
        }
        Err(e) => {
            eprintln!("Error occured {e}");
        }
    }
}

fn are_reports_safe(file_path : &str) -> Result<bool, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file_path)?;

    for line in content.lines() {
        /*let test: Vec<i32> = line.split_whitespace()
            .map(|level| level.parse::<i32>())
            .collect();

        println!("{test:?}")*/
    }

    Ok(false)
}