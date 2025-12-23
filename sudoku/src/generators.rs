use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::validators::is_valid;

pub fn generate_grid() -> [[usize; 9]; 9] {
    let seed = generate_first_row();
    let shift = |offset: usize| {
        let mut row = [0; 9];
        for i in 0..9 {
            row[i] = seed[(i + offset) % 9];
        }
        row
    };

    [
        seed,
        shift(3),
        shift(6),
        shift(1),
        shift(4),
        shift(7),
        shift(2),
        shift(5),
        shift(8),
    ]
}

fn generate_first_row() -> [usize; 9] {
    let mut rng = thread_rng();
    let mut row = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    row.shuffle(&mut rng);
    row
}

/// Remove numbers from a filled grid to create a puzzle
pub fn remove_numbers(grid: &mut [[usize; 9]; 9]) {
    let mut rng = thread_rng();
    let mut indexes: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    indexes.shuffle(&mut rng);

    for i in indexes {
        for j in indexes {
            if num_possibilities(*grid, i, j) <= 1 {
                grid[i][j] = 0;
            }
        }
    }
}

/// Count how many numbers are valid placements at this position
fn num_possibilities(grid: [[usize; 9]; 9], y: usize, x: usize) -> u32 {
    let mut grid = grid;
    grid[y][x] = 0; // Clear the cell to check possibilities

    (1..=9).filter(|&n| is_valid(grid, y, x, n)).count() as u32
}

#[cfg(test)]
mod tests {
  use super::*;
  
  static GRID: [[usize; 9]; 9] = [
    [1,0,0,0,0,0,0,0,0],
    [0,2,0,0,0,0,0,0,0],
    [0,0,3,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,6,0,0,0],
    [0,0,0,0,0,0,7,0,0],
    [0,0,0,0,0,0,0,8,9],
  ];

  #[test]
  fn counts_num_possibilities() {
    // Position (0, 5): blocked by 1 (row) and 6 (column), so 7 possibilities
    assert_eq!(num_possibilities(GRID, 0, 5), 7);
  }

  #[test]
  fn counts_zero_possibilities_when_all_blocked() {
    let grid: [[usize; 9]; 9] = [
      [0, 2, 3, 4, 5, 6, 7, 8, 9], // Position (0,0) has no valid options
      [1, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(num_possibilities(grid, 0, 0), 0);
  }
}