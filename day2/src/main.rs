use eyre::Result;
use std::{fs::File, num::ParseIntError};

fn read_csv_records() -> Result<Vec<String>> {
    let mut list: Vec<String> = [].to_vec();
    let file = File::open("puzzle.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    for result in rdr.records() {
        let record = result?;
        for field in record.iter() {
            list.push(field.to_string());
        }
    }
    Ok(list)
}

#[inline]
fn parse_one_range(nums: &str) -> (String, String) {
    let (left, right) = nums.split_once('-').expect("This was promised");
    (left.to_string(), right.to_string())
}

fn check_one_range(left: String, right: String) -> u64 {
    let mut counts = 0;
    let left_num: u64 = left.parse().expect("this is promised L");
    let right_num: u64 = right.parse().expect("this is promised R");
    for i in left_num..=right_num {
        let i_str = i.to_string();
        for j in 0..i_str.len() {
            if i_str.len() % (j + 1) == 0 && i_str.len() / (j + 1) != 1 {
                // We can have a match
                let constructed = i_str[0..j + 1].to_string().repeat(i_str.len() / (j + 1));
                if constructed == i_str {
                    println!("{},{}", constructed, i_str);
                    counts += i;
                    break;
                }
            }
        }
    }
    counts
}

fn main() {
    let ranges = read_csv_records().expect("cant read");
    let mut counts = 0;
    for range in ranges {
        let (left, right) = parse_one_range(range.as_str());
        let new_ct = check_one_range(left.clone(), right.clone());
        counts += new_ct;
        println!("{},{},{}", left, right, new_ct)
    }
    println!("{}", counts)
}
