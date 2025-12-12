use eyre::bail;
use eyre::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Reads a file line by line into a Vec<String>.
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn update_count(mut current: i32, line: &String, mut count: i32) -> Result<(i32, i32)> {
    let mag = line[1..].parse::<i32>().expect("This should be a number");
    current = current + 100_000; //So we dont have to deal with negative numbers
                                 // Here we branch
                                 // We know that the string starts with L or R
    let num: Result<i32> = match line.chars().next().expect("This exists") {
        // .unwrap() is safe because we checked line.len()
        'L' => Ok(-mag),
        'R' => Ok(mag),
        _ => bail!("oops"), // The first character was not 'L' or 'R'
    };
    let num: i32 = num.expect("this exists");
    let mut new_current = current + num;
    let crosses = if num > 0 {
        // Moving right: count how many multiples of 100 in (current, new_current]
        new_current / 100 - current / 100
    } else {
        // Moving left: count how many multiples of 100 in [new_current, current)
        (current - 1) / 100 - (new_current - 1) / 100
    };
    Ok((new_current % 100, count + crosses))
}

fn main() {
    let mut current = 50;
    let mut count = 0;
    let fname = "puzzle.txt";
    let instr_str = lines_from_file(fname).expect("This should exist");
    for line in instr_str.iter() {
        if !line.is_empty() {
            let prev = current;
            (current, count) = update_count(current, line, count).expect("This exists");
            // count += (current / 100 - prev / 100).abs();
            println!("{}, {}, {}", line, current, count)
        }
    }
}
