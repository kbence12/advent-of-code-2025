use indicatif::ProgressBar;
use ndarray::Array2;
use rayon::prelude::*;
use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point2d {
    x: i32,
    y: i32,
}

impl From<(&str, &str)> for Point2d {
    fn from(inpt: (&str, &str)) -> Point2d {
        Point2d {
            x: inpt.0.to_owned().parse().unwrap(),
            y: inpt.1.to_owned().parse().unwrap(),
        }
    }
}

impl Point2d {
    fn area_with(&self, other: &Point2d) -> u64 {
        (((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1))
            .try_into()
            .unwrap() //We can unwrap since it is after abs.
    }
}

/// Build a set of all green tiles (edges between consecutive red tiles)
fn build_green_edges(red_tiles: &[Point2d]) -> HashSet<Point2d> {
    let mut green = HashSet::new();

    for i in 0..red_tiles.len() {
        let start = red_tiles[i];
        let end = red_tiles[(i + 1) % red_tiles.len()];

        // Add all tiles on the line segment between start and end (exclusive of endpoints)
        if start.x == end.x {
            // Vertical line
            let min_y = min(start.y, end.y);
            let max_y = max(start.y, end.y);
            for y in min_y..=max_y {
                green.insert(Point2d { x: start.x, y });
            }
        } else if start.y == end.y {
            // Horizontal line
            let min_x = min(start.x, end.x);
            let max_x = max(start.x, end.x);
            for x in min_x..=max_x {
                green.insert(Point2d { x, y: start.y });
            }
        }
    }

    green
}

/// Check if a point is inside the polygon using ray casting algorithm
fn point_in_polygon(point: &Point2d, polygon: &[Point2d]) -> bool {
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let j = (i + 1) % n;
        let vi = polygon[i];
        let vj = polygon[j];

        if ((vi.y > point.y) != (vj.y > point.y))
            && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
        {
            inside = !inside;
        }
    }

    inside
}

/// Build a set of all green tiles (edges + interior)
fn build_all_green_tiles(red_tiles: &[Point2d]) -> HashSet<Point2d> {
    let mut green = build_green_edges(red_tiles);

    // Find bounding box
    let min_x = red_tiles.iter().map(|p| p.x).min().unwrap();
    let max_x = red_tiles.iter().map(|p| p.x).max().unwrap();
    let min_y = red_tiles.iter().map(|p| p.y).min().unwrap();
    let max_y = red_tiles.iter().map(|p| p.y).max().unwrap();

    // Check all points in bounding box in parallel
    let pb = ProgressBar::new((max_x - min_x + 1) as u64);
    let interior_points: Vec<Point2d> = (min_x..=max_x)
        .into_par_iter()
        .flat_map(|x| {
            let points: Vec<Point2d> = (min_y..=max_y)
                .filter_map(|y| {
                    let point = Point2d { x, y };
                    if point_in_polygon(&point, red_tiles) {
                        Some(point)
                    } else {
                        None
                    }
                })
                .collect();
            pb.inc(1);
            points
        })
        .collect();
    pb.finish();

    green.extend(interior_points);
    green
}

fn is_valid(a: usize, b: usize, db: &[Point2d]) -> bool {
    let lb_x = min(db[a].x, db[b].x);
    let ub_x = max(db[a].x, db[b].x);
    let lb_y = min(db[a].y, db[b].y);
    let ub_y = max(db[a].y, db[b].y);
    !db.iter().any(|point| {
        let x_inside = (lb_x < point.x && point.x < ub_x);
        let y_inside = (lb_y < point.y && point.y < ub_y);
        x_inside && y_inside
    })
}

fn is_valid_part2(
    a: usize,
    b: usize,
    red_tiles: &[Point2d],
    valid_tiles: &HashSet<Point2d>,
) -> bool {
    let lb_x = min(red_tiles[a].x, red_tiles[b].x);
    let ub_x = max(red_tiles[a].x, red_tiles[b].x);
    let lb_y = min(red_tiles[a].y, red_tiles[b].y);
    let ub_y = max(red_tiles[a].y, red_tiles[b].y);

    // Check all tiles in the rectangle (including boundaries)
    // Single hash lookup instead of two
    for x in lb_x..=ub_x {
        for y in lb_y..=ub_y {
            let point = Point2d { x, y };
            if !valid_tiles.contains(&point) {
                return false;
            }
        }
    }

    true
}

fn array_dists(points: Vec<Point2d>, arr: &mut Array2<u64>) -> (usize, usize) {
    let mut prev_max = 0;
    let mut index = Point2d { x: 0, y: 0 }; //This is an abuse but okay for now
    for i in 0..points.len() {
        for j in i..points.len() {
            let valid = is_valid(i, j, &points);
            let area = if valid {
                points[i].area_with(&points[j])
            } else {
                0
            };
            arr[[i, j]] = area;
            if area > prev_max {
                prev_max = area;
                index = Point2d {
                    x: i as i32,
                    y: j as i32,
                };
            }
        }
    }
    (index.x.try_into().unwrap(), index.y.try_into().unwrap())
}

fn array_dists_part2(
    points: &[Point2d],
    valid_tiles: &HashSet<Point2d>,
    arr: &mut Array2<u64>,
) -> (usize, usize) {
    let pb = ProgressBar::new(points.len() as u64);

    // Parallelize the computation
    let results: Vec<(usize, usize, u64)> = (0..points.len())
        .into_par_iter()
        .flat_map(|i| {
            let row_results: Vec<(usize, usize, u64)> = (i..points.len())
                .map(|j| {
                    let valid = is_valid_part2(i, j, points, valid_tiles);
                    let area = if valid {
                        points[i].area_with(&points[j])
                    } else {
                        0
                    };
                    (i, j, area)
                })
                .collect();
            pb.inc(1);
            row_results
        })
        .collect();

    pb.finish();

    // Find max and populate array
    let mut prev_max = 0;
    let mut index = (0, 0);
    for (i, j, area) in results {
        arr[[i, j]] = area;
        if area > prev_max {
            prev_max = area;
            index = (i, j);
        }
    }

    index
}

fn main() {
    let contents = fs::read_to_string("test.txt").expect("Failed to read test.txt");
    let parsed: Vec<Point2d> = contents
        .lines()
        .map(|s| s.split_once(",").map(|p| p.into()).unwrap())
        .collect();

    // Part 2: Build red and green tiles sets
    let red_set: HashSet<Point2d> = parsed.iter().copied().collect();
    let green_tiles = build_all_green_tiles(&parsed);

    // Merge red and green tiles for faster validation (single hash lookup)
    let mut valid_tiles = red_set.clone();
    valid_tiles.extend(&green_tiles);

    let mut dists: Array2<u64> = Array2::zeros((parsed.len(), parsed.len()));
    let coords = array_dists_part2(&parsed, &valid_tiles, &mut dists);

    println!("Part 2 Answer: {}", dists[[coords.0, coords.1]]);
}
