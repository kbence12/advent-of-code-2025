use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const JOLT_MAX_LEN: usize = 12;

/// Reads a file line by line, converting each line of digits into a Vec<u8>.
///
/// Each line in the file is expected to contain a sequence of digits.
/// The function returns a `Vec<Vec<u8>>`, where each inner vector
/// represents a line and contains the digits as `u8` values.
fn read_numbers_from_file<P: AsRef<Path>>(filepath: P) -> io::Result<Vec<Vec<u8>>> {
    // Open the file and create a buffered reader for efficiency.
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    // Use the lines() iterator to read the file line by line.
    // Each line is a Result<String, io::Error>, so we need to handle potential errors.
    reader
        .lines()
        .map(|line| {
            // Ensure the line was read correctly.
            let line = line?;

            // For each character in the line, try to convert it to a digit.
            // We use flat_map to filter out any non-digit characters gracefully.
            // The character is converted to a u8 digit in base 10.
            let digits: Vec<u8> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect();

            // Wrap the result in Ok for the outer collection.
            Ok(digits)
        })
        .collect() // Collect the results into a Vec<Vec<u8>>, handling any errors.
}

fn one_line_max(line: &Vec<u8>) -> u64 {
    let mut vec_of_nums: Vec<u64> = vec![];
    let mut last_max_index: Option<usize> = None;
    let mut remaining_vec = line.clone();

    while vec_of_nums.len() < JOLT_MAX_LEN {
        remaining_vec = remaining_vec[last_max_index.unwrap_or(0)..].to_vec();
        // println!("{:?}", remaining_vec);
        // println!("{:?}", vec_of_nums);
        // println!(
        //     "{}",
        //     (remaining_vec.len() + vec_of_nums.len() - JOLT_MAX_LEN)
        // );
        let working_vec =
            &remaining_vec[..remaining_vec.len() + vec_of_nums.len() + 1 - JOLT_MAX_LEN];
        println!("{:?}", working_vec);
        vec_of_nums.push(
            (*working_vec
                .iter()
                .max()
                .expect("loop vector exists and has elements"))
            .into(),
        );
        last_max_index = Some(
            working_vec
                .iter()
                .max()
                .and_then(|max_val| working_vec.iter().position(|&x| x == *max_val))
                .expect("this is fine")
                + 1,
        );
        println!("{:?}, {:?}", last_max_index, vec_of_nums)
    }
    vec_of_nums
        .iter()
        .fold(0, |acc, &digit| acc * 10 + digit as u64)
}

fn main() {
    let file_path = "puzzle.txt";

    // Call the function to read and parse the file.
    let db = read_numbers_from_file(file_path).expect("unable to read correctly");
    let charges: Vec<u64> = db.iter().map(|vec| one_line_max(vec)).collect();
    println!("{:?}", charges);
    println!("{:?}", charges.into_iter().sum::<u64>())
}
