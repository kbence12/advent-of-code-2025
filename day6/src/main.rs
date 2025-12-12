use eyre::Result;
use eyre::bail;
use std::fs;

fn transpose_consuming(v: Vec<Vec<String>>) -> Vec<Vec<String>> {
    if v.is_empty() {
        return Vec::new();
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|inner| inner.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|inner| inner.next().unwrap())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn one_op(mut line: Vec<String>) -> Result<i64> {
    let op: String = line.pop().expect("non-empty");
    let nums: Vec<i64> = line
        .into_iter()
        .map(|item| item.parse().expect("these are all numbers"))
        .collect();
    match op.as_str() {
        "+" => Ok(nums.iter().sum()),
        "*" => Ok(nums.iter().product()),
        _ => bail!("Oops"),
    }
}

pub enum Op {
    ADD,
    MULT,
    SPACE,
}

struct Part2 {
    operator: Op,
    prior_state: i64,
}

impl Part2 {
    pub fn parse_next(&mut self, mut line: String) -> i64 {
        if line.trim().is_empty() {
            return 0;
        }
        println!("{:?}", line);
        let maybe_new_op = line.pop().expect("non-empty").to_string();
        match maybe_new_op.as_str() {
            "+" => {
                let last_result = self.prior_state;
                self.operator = Op::ADD;
                self.prior_state = line.trim().parse().expect("This is now a number");
                last_result
            }
            "*" => {
                let last_result = self.prior_state;
                self.operator = Op::MULT;
                self.prior_state = line.trim().parse().expect("This is now a number");
                last_result
            }
            " " => match self.operator {
                Op::ADD => {
                    self.prior_state += line.trim().parse::<i64>().expect("This is now a number");
                    0
                }
                Op::MULT => {
                    self.prior_state *= line.trim().parse::<i64>().expect("This is a number");
                    0
                }
                Op::SPACE => {
                    panic!("This is baad");
                }
            },
            _ => panic!("it must be one of the three above"),
        }
    }
}

fn main() {
    // Read the puzzle file
    let contents = fs::read_to_string("puzzle.txt").expect("Failed to read puzzle.txt");

    // Part 1 parse and solve

    // Parse into Vec<Vec<String>>
    //
    // let parsed: Vec<Vec<String>> = contents
    //     .lines()
    //     .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
    //     .collect();
    // let transposed = transpose_consuming(parsed);

    // // Print the result to verify
    // let sum: i64 = transposed
    //     .iter()
    //     .map(|op| one_op(op.to_vec()))
    //     .collect::<Result<Vec<i64>>>()
    //     .expect("Failed to calculate operations")
    //     .iter()
    //     .sum();

    // Parse character-by-character into columns
    let lines: Vec<&str> = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    if lines.is_empty() {
        println!("No data");
        return;
    }

    // Find the maximum line length
    let max_len = lines.iter().map(|line| line.len()).max().unwrap();

    // Create columns by reading each character position across all lines
    let columns: Vec<String> = (0..max_len)
        .map(|col_idx| {
            lines
                .iter()
                .map(|line| line.chars().nth(col_idx).unwrap_or(' '))
                .collect()
        })
        .collect();

    let mut part2 = Part2 {
        operator: Op::SPACE,
        prior_state: 0,
    };
    let mut sum: i64 = columns
        .iter()
        .map(|col| part2.parse_next(col.to_owned()))
        .collect::<Vec<i64>>()
        .iter()
        .sum();
    sum += part2.prior_state;
    println!("{}", sum)
}
