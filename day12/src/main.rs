use std::fs;

#[derive(Debug, Clone)]
struct Shape {
    id: usize,
    max_size: usize, // rows * columns
    min_size: usize, // number of '#' characters
    shape: Vec<Vec<char>>,
}

#[derive(Debug)]
struct GridObject {
    dimensions: (usize, usize), // (rows, cols) from "RxC"
    values: Vec<i32>,
}

fn parse_input(contents: &str) -> (Vec<Shape>, Vec<GridObject>) {
    let lines: Vec<&str> = contents.lines().collect();
    let mut shapes = Vec::new();
    let mut grid_objects = Vec::new();

    let mut i = 0;

    // Parse shapes section
    while i < lines.len() {
        let line = lines[i].trim();

        // Check if this is a grid object line (contains 'x' and ':')
        if line.contains('x') && line.contains(':') {
            // We've reached the grid section
            break;
        }

        // Check if this is a shape header (number followed by ':')
        if line.ends_with(':')
            && !line.is_empty()
            && let Ok(id) = line.trim_end_matches(':').parse::<usize>()
        {
            // Parse the shape
            i += 1;
            let mut shape_lines = Vec::new();

            while i < lines.len() {
                let shape_line = lines[i];
                if shape_line.trim().is_empty() {
                    break;
                }
                // Check if we've hit a grid line
                if shape_line.contains('x') && shape_line.contains(':') {
                    break;
                }
                shape_lines.push(shape_line.chars().collect::<Vec<char>>());
                i += 1;
            }

            if !shape_lines.is_empty() {
                let rows = shape_lines.len();
                let cols = shape_lines[0].len();
                let max_size = rows * cols;
                let min_size = shape_lines
                    .iter()
                    .flat_map(|row| row.iter())
                    .filter(|&&c| c == '#')
                    .count();

                shapes.push(Shape {
                    id,
                    max_size,
                    min_size,
                    shape: shape_lines,
                });
            }
        }

        i += 1;
    }

    // Parse grid objects section
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        if let Some((dims, vals)) = line.split_once(':')
            && let Some((rows_str, cols_str)) = dims.trim().split_once('x')
            && let (Ok(rows), Ok(cols)) = (rows_str.parse::<usize>(), cols_str.parse::<usize>())
        {
            let values: Vec<i32> = vals
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            grid_objects.push(GridObject {
                dimensions: (rows, cols),
                values,
            });
        }

        i += 1;
    }

    (shapes, grid_objects)
}

fn main() {
    let contents = fs::read_to_string("puzzle.txt").expect("Failed to read file");
    let (shapes, grid_objects) = parse_input(&contents);

    println!("Shapes:");
    for shape in &shapes {
        println!(
            "  Shape {}: max_size={}, min_size={}",
            shape.id, shape.max_size, shape.min_size
        );
        for row in &shape.shape {
            println!("    {}", row.iter().collect::<String>());
        }
    }

    println!("\nGrid Objects:");
    for grid in &grid_objects {
        println!(
            "  {}x{}: {:?}",
            grid.dimensions.0, grid.dimensions.1, grid.values
        );
    }

    let mut def_fits = 0;
    let mut def_not_fit = 0;
    let mut hard = 0;
    grid_objects.into_iter().for_each(|grid| {
        let mut required_area_min = 0;
        let mut required_area_max = 0;
        for (idx, num) in grid.values.iter().enumerate() {
            required_area_max += num * shapes[idx].max_size as i32;
            required_area_min += num * shapes[idx].min_size as i32;
        }
        if required_area_max <= (grid.dimensions.0 * grid.dimensions.1) as i32 {
            def_fits += 1
        } else if required_area_min >= (grid.dimensions.0 * grid.dimensions.1) as i32 {
            def_not_fit += 1
        } else {
            hard += 1
        }
    });

    //TODO:
    // At some point try to code up the hard bit where we need to rotate things and get the smallest fully filled
    // rectangle ones.
    println!(
        "easy fit: {},easy not fit {}, hard {}",
        def_fits, def_not_fit, hard
    )
}
