use anyhow::{Context, Result};
use std::time::Instant;
use std::cmp;

/// Entry point for the program
///
/// This program finds all multiplications in the input, performs the multiplication and sum the results up
fn main() {
    let start = Instant::now();

    let file_path = "src/input.txt";
    let result = count_word_occurence_in_word_search(&file_path, "XMAS");

    match result {
        Ok(word_count) => {
            println!("Total word occurrences: {word_count}");
        }
        Err(e) => {
            eprintln!("Error occured: {e:?}");
        }
    }

    let duration = start.elapsed();
    println!("Execution time: {:.2?}", duration);
}

fn count_word_occurence_in_word_search(file_path: &str, word: &str) -> Result<i32> {
    let content = std::fs::read_to_string(file_path)
    .with_context(|| format!("Failed to read file: {file_path}"))?;

    let matrix: Vec<Vec<char>> = content.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut occurences: i32 = 0;
    occurences += count_word_occurence_in_matrix(matrix.clone(), word);
    occurences += count_word_occurence_in_matrix(get_vertical_lines(&matrix), word);
    occurences += count_word_occurence_in_matrix(get_forward_diagonal_lines(&matrix), word);
    occurences += count_word_occurence_in_matrix(get_backward_diagonal_lines(&matrix), word);

    Ok(occurences)
}

fn get_vertical_lines(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_rows: usize = matrix.len();
    let num_columns = matrix[0].len();
    let mut result: Vec<Vec<char>> = Vec::new();

    for i in 0..num_columns {
        let mut line: Vec<char> = Vec::new();
        for j in 0..num_rows {
            line.push(matrix[j][i]);
        }
        result.push(line);
    }

    return result;
}

fn get_backward_diagonal_lines(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_rows: usize = matrix.len();
    let num_columns = matrix[0].len();
    let mut result: Vec<Vec<char>> = Vec::new();
    for i in (0..num_rows).rev() {
        let mut line: Vec<char> = Vec::new();
        for j in 0..cmp::max(num_columns, num_rows) {
            if i+j > num_rows-1 || j > num_columns-1 {
                break;
            }
            line.push(matrix[i+j][j])
        }
        result.push(line);
    }
    for i in 1..num_columns {
        let mut line: Vec<char> = Vec::new();
        for j in 0..cmp::max(num_columns, num_rows) {
            if j > num_rows-1 || i+j > num_columns-1 {
                break;
            }
            line.push(matrix[j][i+j]);
        }
        result.push(line);
    }
    return result;
}

fn get_forward_diagonal_lines(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_rows: usize = matrix.len();
    let num_columns = matrix[0].len();
    let mut result: Vec<Vec<char>> = Vec::new();
    for i in 0..num_rows {
        let mut line: Vec<char> = Vec::new();
        for j in 0..cmp::max(num_columns, num_rows) {
            if j > num_columns-1 || j > i {
                break;
            }
            line.push(matrix[i-j][j])
        }
        result.push(line);
    }
    for i in 1..num_columns {
        let mut line: Vec<char> = Vec::new();
        for j in 0..num_rows {
            if j > num_rows-1 || i+j > num_columns-1 {
                break;
            }
            line.push(matrix[num_rows-1-j][i+j]);
        }
        result.push(line);
    }
    return result;
}

fn count_word_occurence_in_matrix(matrix: Vec<Vec<char>>, word: &str) -> i32 {
    //let result: Vec<i32> = matrix;
    let result: i32 = matrix
        .iter()
        .map(|vector| {
            count_word_occurrence_in_vector(vector, word)
        })
        .sum();
    return result;
}

fn count_word_occurrence_in_vector(line: &Vec<char>, word: &str) -> i32 {
    let result : i32 = line
        .windows(word.len())
        .filter(|window| window.iter().copied().eq(word.chars()) || window.iter().rev().copied().eq(word.chars()))    
        .count() as i32;

    return result;
}
