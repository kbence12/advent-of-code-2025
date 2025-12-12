use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct IntRange {
    mn: u64,
    mx: u64,
}

#[derive(Clone)]
struct Puzzle {
    pub codes: Vec<IntRange>,
    pub ids: Vec<u64>,
}

impl IntRange {
    pub fn is_in_range(&self, item: u64) -> bool {
        self.mn <= item && self.mx >= item
    }
}

impl Puzzle {
    pub fn load<P: AsRef<Path>>(filepath: P) -> Puzzle {
        let file = File::open(filepath).expect("Wrong filepath");
        let reader = BufReader::new(file);

        let mut puz = Puzzle {
            codes: vec![],
            ids: vec![],
        };
        // Use the lines() iterator to read the file line by line.
        // Each line is a Result<String, io::Error>, so we need to handle potential errors.
        reader.lines().for_each(|line| {
            // Ensure the line was read correctly.
            let line = line.expect("F up line");
            // println!("{}", line);
            if let Some(strs) = line.split_once("-") {
                let lower: u64 = strs.0.parse().expect("this is a number");
                let upper: u64 = strs.1.parse().expect("This is also a number");
                puz.codes.push(IntRange {
                    mn: lower,
                    mx: upper,
                });
            } else if !line.is_empty() {
                puz.ids
                    .push(line.parse().expect("I was promised this is a number"));
            }
        });
        puz.codes.sort();
        puz
    }

    pub fn num_fresh(&self) -> u64 {
        let mut count = 0;

        self.ids.clone().into_iter().for_each(|idx| {
            let mut inside_count: u64 = 0;
            self.codes
                .clone()
                .into_iter()
                .for_each(|range| inside_count += range.is_in_range(idx) as u64);
            if inside_count > 0 {
                count += 1;
            }
        });
        count
    }

    pub fn all_fresh_ids(self) -> u64 {
        let mut count = 0;
        let mut last_max = 0;
        self.codes.into_iter().for_each(|rn| {
            println!("{},{}", rn.mn, rn.mx);
            if rn.mn > last_max {
                //Happy days
                count += rn.mx - rn.mn + 1;
                last_max = rn.mx;
            } else if rn.mx > last_max {
                // We only need to add a range if this the top is outside the prev one
                count += rn.mx - last_max;
                last_max = rn.mx;
            }
            println!("{}", last_max)
        });
        count
    }
}

fn main() {
    let fpath = "puzzle.txt";
    let puzzle = Puzzle::load(fpath);
    let _num = puzzle.num_fresh();
    let num = puzzle.all_fresh_ids();
    println!("{}", num)
}
