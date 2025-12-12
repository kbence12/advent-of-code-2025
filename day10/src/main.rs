use good_lp::*;
use std::collections::{HashSet, VecDeque};
use std::fs;

fn xor_vectors(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

pub fn find_minimum_instructions(target: &[u8], instructions: &[Vec<u8>]) -> Option<usize> {
    let n = target.len();
    if n == 0 {
        return Some(0);
    }
    let start_state = vec![0; n];
    if target == start_state.as_slice() {
        return Some(0);
    }

    let mut queue: VecDeque<(Vec<u8>, usize)> = VecDeque::new();

    let mut visited: HashSet<Vec<u8>> = HashSet::new();

    queue.push_back((start_state.clone(), 0));
    visited.insert(start_state);

    while let Some((current_vec, dist)) = queue.pop_front() {
        for instruction in instructions {
            let next_vec = xor_vectors(&current_vec, instruction);

            if next_vec.as_slice() == target {
                return Some(dist + 1);
            }

            if !visited.contains(&next_vec) {
                visited.insert(next_vec.clone());
                queue.push_back((next_vec, dist + 1));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct ParsedLine {
    pattern: Vec<u8>,
    groups: Vec<Vec<u8>>,
    target: Option<Vec<u16>>,
}

fn parse_pattern(s: &str) -> Vec<u8> {
    s.chars()
        .filter_map(|c| match c {
            '.' => Some(0),
            '#' => Some(1),
            _ => None,
        })
        .collect()
}

fn parse_group(s: &str, length: usize) -> Vec<u8> {
    let mut result = vec![0; length];

    // Remove parentheses and parse comma-separated numbers
    let inner = s.trim_start_matches('(').trim_end_matches(')');

    for num_str in inner.split(',') {
        if let Ok(num) = num_str.trim().parse::<usize>() {
            result[num] = 1;
        }
    }

    result
}

fn parse_target(s: &str) -> Vec<u16> {
    // Remove braces and parse comma-separated numbers
    let inner = s.trim_start_matches('{').trim_end_matches('}');

    inner
        .split(',')
        .filter_map(|num_str| num_str.trim().parse::<u16>().ok())
        .collect()
}

fn parse_line(line: &str) -> Option<ParsedLine> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Find the pattern [...]
    let pattern_start = line.find('[')?;
    let pattern_end = line.find(']')?;
    let pattern_str = &line[pattern_start + 1..pattern_end];
    let pattern = parse_pattern(pattern_str);

    // Find all groups (...) and target {...}
    let mut groups = Vec::new();
    let mut target = None;
    let rest = &line[pattern_end + 1..];

    let chars = rest.chars().peekable();
    let mut current_group = String::new();
    let mut current_target = String::new();
    let mut in_group = false;
    let mut in_braces = false;

    for c in chars {
        match c {
            '(' if !in_braces => {
                in_group = true;
                current_group.push(c);
            }
            ')' if in_group && !in_braces => {
                current_group.push(c);
                groups.push(parse_group(&current_group, pattern.len()));
                current_group.clear();
                in_group = false;
            }
            '{' => {
                in_braces = true;
                current_target.push(c);
            }
            '}' => {
                current_target.push(c);
                target = Some(parse_target(&current_target));
                current_target.clear();
                in_braces = false;
            }
            _ if in_braces => {
                current_target.push(c);
            }
            _ if in_group => {
                current_group.push(c);
            }
            _ => {}
        }
    }

    Some(ParsedLine {
        pattern,
        groups,
        target,
    })
}

fn parse_file(filename: &str) -> Vec<ParsedLine> {
    let content = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    content.lines().filter_map(parse_line).collect()
}

pub fn find_minimum_instructions_lp(target: &[u16], instructions: &[Vec<u8>]) -> Option<usize> {
    let mut problem = ProblemVariables::new();

    // Create integer variables for each instruction (how many times to use it)
    let usage_vars: Vec<Variable> = (0..instructions.len())
        .map(|i| problem.add(variable().integer().min(0).name(format!("x{}", i))))
        .collect();

    // Objective: minimize the sum of all usage variables
    let objective: Expression = usage_vars
        .iter()
        .fold(Expression::from(0.0), |acc, &var| acc + var);

    // Build the solver
    let mut solver = problem.minimise(objective).using(default_solver);

    // Add constraints for each target position
    for (pos_idx, &target_val) in target.iter().enumerate() {
        let mut expr = Expression::from(0.0);

        for (instr_idx, instruction) in instructions.iter().enumerate() {
            let coeff = instruction[pos_idx] as f64;
            if coeff > 0.0 {
                expr += coeff * usage_vars[instr_idx];
            }
        }

        // Constraint: sum must equal target value
        solver = solver.with(constraint!(expr == target_val as f64));
    }

    // Solve the problem
    let solution = solver.solve();

    match solution {
        Ok(sol) => {
            let mut total = 0;
            for &var in &usage_vars {
                let val = sol.value(var);
                total += val.round() as usize;
            }
            Some(total)
        }
        Err(_) => None,
    }
}

fn main() {
    // Parse puzzle.txt
    println!("=== Parsing puzzle.txt ===");
    let puzzle_data = parse_file("puzzle.txt");

    println!("\n=== SOLUTION 1: XOR with pattern as target ===");
    let mut summed_xor = 0;
    for line in puzzle_data.iter() {
        let num = find_minimum_instructions(&line.pattern, &line.groups).unwrap();
        summed_xor += num;
        //println!();
    }
    println!("Total (XOR): {}", summed_xor);

    println!("\n=== SOLUTION 4: ILP Solver with {{}} as target ===");
    let mut summed_lp = 0;
    for (i, line) in puzzle_data.iter().enumerate() {
        if let Some(ref target) = line.target {
            match find_minimum_instructions_lp(target, &line.groups) {
                Some(num) => {
                    println!("Line {}: {:?} -> {}", i + 1, target, num);
                    summed_lp += num;
                }
                None => {
                    println!("Line {}: {:?} -> NO SOLUTION FOUND", i + 1, target);
                    println!("  Groups: {:?}", line.groups);
                }
            }
        }
    }
    println!("Total (ILP): {}", summed_lp);
}
