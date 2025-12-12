use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("puzzle.txt").expect("Failed to read puzzle.txt");
    let mut parsed: Vec<Vec<char>> = contents
        .lines()
        .map(|s| s.to_string().as_str().chars().collect())
        .collect();
    // I actually think I need a for loop here
    let string_line: String = parsed[0].clone().into_iter().collect();
    let start_pos = string_line.find("S").unwrap(); //This is fine, first line has an S
    // Thats a hack
    parsed[1][start_pos - 1] = '|';
    parsed[1][start_pos + 1] = '|';
    let mut pos_all = HashMap::new();
    pos_all.insert(start_pos - 1, 1_u64);
    pos_all.insert(start_pos + 1, 1_u64);
    let mut i = 4;
    let mut count = 1;
    while i < parsed.len() {
        // while i < 10 {
        //^ line, and then match with prior positions
        let line_string: String = parsed[i].clone().into_iter().collect();
        let mut new_indices: Vec<usize> = line_string
            .as_str()
            .match_indices("^")
            .map(|(index, _)| index)
            .collect();
        // println!("{:?}", new_indices);
        new_indices.retain(|element| pos_all.contains_key(element));
        count += new_indices.len() as i32;
        // println!("{:?}", count);
        //Here add to keep count of colliders count.
        new_indices.clone().into_iter().for_each(|item| {
            let prior_count = pos_all.remove(&item).unwrap().to_owned(); //We have checked above that this exists
            let score_plus = pos_all.entry(item + 1).or_insert(0);
            *score_plus += prior_count;
            let score_minus = pos_all.entry(item - 1).or_insert(0);
            *score_minus += prior_count;
        });
        println!("{:?}", pos_all);
        i += 2;
    }
    println!("{}", pos_all.len());
    println!("{}", count);
    println!("{}", pos_all.values().sum::<u64>())
}
