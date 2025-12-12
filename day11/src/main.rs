use std::{collections::HashMap, fs};

use petgraph::algo::is_cyclic_directed;
use petgraph::graph::DiGraph;
use petgraph::prelude::NodeIndex;

fn get_total_paths(
    graph: &DiGraph<u32, ()>,
    name_map: &HashMap<String, u32>,
    start_name: &str,
    end_name: &str,
) -> u64 {
    // 1. Find the start and end NodeIndex values from the names.
    let start_node = name_map
        .get(start_name)
        .expect("not found start")
        .to_owned();
    let end_node = name_map.get(end_name).expect("not found end").to_owned();

    // 2. Create a cache for memoization. The key is the node we are starting from,
    //    and the value is the number of paths from it to the final `end_node`.
    let mut cache = HashMap::<NodeIndex, u64>::new();

    // 3. Call the internal recursive function to do the actual work.
    count_paths_recursive(graph, start_node.into(), end_node.into(), &mut cache)
}

/// Recursive function with memoization to count paths.
fn count_paths_recursive(
    graph: &DiGraph<u32, ()>,
    current_node: NodeIndex,
    end_node: NodeIndex,
    cache: &mut HashMap<NodeIndex, u64>,
) -> u64 {
    // Base Case 1: If we have reached the end node, we've found one valid path.
    if current_node == end_node {
        return 1;
    }

    // Base Case 2 (Memoization): If we have already calculated the number of paths
    // from `current_node`, return the cached result immediately.
    if let Some(&count) = cache.get(&current_node) {
        return count;
    }

    // Recursive Step: The total paths from `current_node` is the sum of the paths
    // from all its neighbors.
    let mut path_count: u64 = 0;
    graph
        .neighbors(current_node)
        .for_each(|neighbor| path_count += count_paths_recursive(graph, neighbor, end_node, cache));

    // After calculating, store the result in the cache before returning.
    cache.insert(current_node, path_count);

    path_count
}

fn count_paths_via_sequence(
    graph: &DiGraph<u32, ()>,
    name_map: &HashMap<String, u32>,
    start_name: &str,
    via_names: &[&str],
    end_name: &str,
) -> u64 {
    // 1. Create the full chain of nodes to visit in order.
    let mut path_chain = Vec::with_capacity(via_names.len() + 2);
    path_chain.push(start_name);
    path_chain.extend_from_slice(via_names);
    path_chain.push(end_name);

    // 2. Initialize total paths to 1 for multiplication.
    let mut total_paths: u64 = 1;

    // 3. Iterate through the chain in overlapping pairs (segments).
    //    e.g., [start, via1, via2, end] -> [start, via1], [via1, via2], [via2, end]
    for segment in path_chain.windows(2) {
        let segment_start = segment[0];
        let segment_end = segment[1];

        // Calculate paths for this specific segment.
        let segment_paths = get_total_paths(graph, name_map, segment_start, segment_end);

        // If any segment is impossible, the whole chain is impossible.
        if segment_paths == 0 {
            return 0;
        }

        // Multiply the total by the number of ways this segment can be traversed.
        total_paths *= segment_paths;
    }

    total_paths
}

fn main() {
    let contents = fs::read_to_string("test_2.txt").expect("Failed to read puzzle.txt");
    let edges: Vec<(String, String)> = contents
        .lines()
        .flat_map(|line| {
            let (source, targets) = line.split_once(":").expect("readme promised");
            let tv: Vec<String> = targets.split_whitespace().map(|s| s.to_string()).collect();
            tv.into_iter()
                .map(|dest| (source.to_owned(), dest))
                .collect::<Vec<(String, String)>>()
        })
        .collect();
    // Create a mapping from String node names to u32 IDs
    let mut name_to_id = HashMap::<String, u32>::new();
    let mut next_id = 0u32;

    // First pass: assign IDs to all unique nodes
    for (source, target) in &edges {
        name_to_id.entry(source.clone()).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        name_to_id.entry(target.clone()).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
    }

    // Second pass: create edges vector with mapped u32 values
    let mapped_edges: Vec<(u32, u32)> = edges
        .iter()
        .map(|(source, target)| {
            (
                *name_to_id.get(source).unwrap(),
                *name_to_id.get(target).unwrap(),
            )
        })
        .collect();
    let gr = DiGraph::<u32, ()>::from_edges(&mapped_edges);
    println!("{}", is_cyclic_directed(&gr));
    let pt = get_total_paths(&gr, &name_to_id, "svr", "out");
    let pt2 = count_paths_via_sequence(&gr, &name_to_id, "svr", &["dac", "fft"], "out")
        + count_paths_via_sequence(&gr, &name_to_id, "svr", &["fft", "dac"], "out");
    println!("Name mapping: {:?}", name_to_id);
    println!("Mapped edges: {:?}", mapped_edges);
    println!("{}", pt2)
}
