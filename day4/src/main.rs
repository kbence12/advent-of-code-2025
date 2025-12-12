use eyre::{bail, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Define the grid dimensions as constants.
// This is the key requirement for using std::array.
const WIDTH: usize = 141;
const HEIGHT: usize = 141;
type Grid = [[u32; WIDTH]; HEIGHT];

/// Reads a file with known dimensions into a 2D std::array.
///
/// This function will return an error if the file does not have
/// exactly HEIGHT lines and WIDTH characters per line.
fn read_grid_to_array<P: AsRef<Path>>(filepath: P) -> Result<[[u32; WIDTH]; HEIGHT]> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    // Initialize the array with a placeholder character.
    // The '\0' (null character) is a common choice.
    let mut grid = [[0; WIDTH]; HEIGHT];
    let mut lines = reader.lines();

    for i in 0..HEIGHT - 2 {
        // Get the next line from the file.
        let line = match lines.next() {
            Some(line_result) => line_result?, // Handle potential I/O errors for the line.
            None => bail!(format!("File has fewer than {} lines", HEIGHT - 2),),
        };

        // Check if the line has the correct number of characters.
        if line.chars().count() != WIDTH - 2 {
            bail!(format!(
                "Line {} has {} characters, but {} were expected",
                i,
                line.chars().count(),
                WIDTH
            ),);
        }

        // Populate the row in our grid.
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '@' => grid[i + 1][j + 1] = 1,
                '.' => grid[i + 1][j + 1] = 0,
                _ => bail!("unexpected char"),
            }
        }
    }

    Ok(grid)
}

fn check_element(col: usize, row: usize, array: &mut Grid) -> bool {
    let inside_count = array[col - 1][row - 1]
        + array[col][row - 1]
        + array[col + 1][row - 1]
        + array[col - 1][row]
        + array[col + 1][row]
        + array[col - 1][row + 1]
        + array[col][row + 1]
        + array[col + 1][row + 1];
    if inside_count < 4 {
        array[col][row] = 0;
        true
    } else {
        false
    }
}

fn main() {
    // Create a dummy file with the correct dimensions (10x5).
    let file_path = "puzzle.txt";
    let (mut count, mut prev_count) = (0, -1);

    let mut grid = read_grid_to_array(file_path).expect("We know the file");
    while prev_count != count {
        prev_count = count;
        for (i, row_array) in grid.clone().iter_mut().enumerate() {
            for (j, _) in row_array.iter_mut().enumerate() {
                //its easier this way
                if i == 0 || j == 0 || i == HEIGHT - 1 || j == WIDTH - 1 || grid[i][j] == 0 {
                    continue;
                } else {
                    count += check_element(i, j, &mut grid) as i32;
                }
            }
        }
        println!("{}", count)
    }
    println!("{}", count)
}
