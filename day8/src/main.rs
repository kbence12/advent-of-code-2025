use core::f64;
use ndarray::Array2;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl TryFrom<&[f64]> for Point {
    type Error = &'static str;

    fn try_from(slice: &[f64]) -> Result<Self, Self::Error> {
        if slice.len() == 3 {
            Ok(Point {
                x: slice[0],
                y: slice[1],
                z: slice[2],
            })
        } else {
            Err("Slice must contain exactly 3 elements")
        }
    }
}

impl Point {
    fn distance_from(&self, p: &Point) -> f64 {
        let sq_dist = (self.x - p.x).powf(2.) + (self.y - p.y).powf(2.) + (self.z - p.z).powf(2.);
        sq_dist.sqrt()
    }
}

fn create_distance_matrix(points: &[Point]) -> Array2<f64> {
    let mut distances = Array2::from_elem((points.len(), points.len()), f64::MAX);
    // Iterate and populate the array
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let dist = points[i].distance_from(&points[j]);
            // Use tuple indexing (i, j)
            distances[[i, j]] = dist;
        }
    }
    distances
}

fn shortest_connections(n: usize, distances: Array2<f64>) -> Vec<(f64, usize, usize)> {
    let mut valid_pairs: Vec<(f64, usize, usize)> = distances
        .indexed_iter()
        // Keep only the pairs that are not f64::MAX
        .filter(|(_, dist)| dist < &&f64::MAX)
        // Remap to (distance, i, j) for easy sorting
        .map(|((i, j), &dist)| (dist, i, j))
        .collect();

    // Sort the collected pairs by distance (the first element of the tuple)
    valid_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Take the first N pairs from the sorted list
    valid_pairs[..n].to_vec()
}
fn count_components(adj_list: &HashMap<usize, Vec<usize>>, total_nodes: usize) -> usize {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut num_components = 0;

    for node in 0..total_nodes {
        if !visited.contains(&node) {
            // Start BFS from this node
            num_components += 1;
            let mut queue = VecDeque::new();
            queue.push_back(node);
            visited.insert(node);

            while let Some(current) = queue.pop_front() {
                if let Some(neighbors) = adj_list.get(&current) {
                    for &neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    num_components
}

fn main() {
    let contents = fs::read_to_string("puzzle.txt").expect("Failed to read puzzle.txt");
    let parsed: Vec<Point> = contents
        .lines()
        .map(|s| {
            Point::try_from(
                s.to_string()
                    .splitn(3, ",")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|num| num.parse::<f64>().expect("Puzzle promised numbers"))
                    .collect::<Vec<f64>>()
                    .as_slice(),
            )
            .expect("Each row has 3 elements")
        })
        .collect();

    let total_nodes = parsed.len();
    let distances = create_distance_matrix(&parsed);

    // Get ALL valid connections sorted by distance
    let mut all_connections: Vec<(f64, usize, usize)> = distances
        .indexed_iter()
        .filter(|(_, dist)| dist < &&f64::MAX)
        .map(|((i, j), &dist)| (dist, i, j))
        .collect();
    all_connections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Start with empty graph
    let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();

    // Initialize all nodes in the adjacency list (even isolated ones)
    for i in 0..total_nodes {
        adj_list.entry(i).or_default();
    }

    // Add connections one by one until fully connected
    let mut connections_added = 0;
    for &(dist, i, j) in &all_connections {
        // Add the connection
        adj_list.entry(i).or_default().push(j);
        adj_list.entry(j).or_default().push(i);
        connections_added += 1;

        // Check if graph is fully connected
        let num_components = count_components(&adj_list, total_nodes);

        if num_components == 1 {
            println!(
                "Graph fully connected after {} connections",
                connections_added
            );
            println!(
                "Last connection added: distance {:.2} between nodes {} and {}",
                dist, i, j
            );
            println!("These nodes are {:?}, {:?}", parsed[i], parsed[j]);
            println!("final product is {}", parsed[i].x * parsed[j].x);
            break;
        }

        if connections_added % 100 == 0 {
            println!(
                "Added {} connections, {} components remaining",
                connections_added, num_components
            );
        }
    }

    // Final verification
    let final_components = count_components(&adj_list, total_nodes);
    println!("Final number of components: {}", final_components);
}
