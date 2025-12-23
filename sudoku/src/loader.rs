pub fn serialize(grid: &[[usize; 9]; 9]) -> String {
    let mut output = String::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            output.push_str(&cell.to_string());
            if j == 2 || j == 5 {
                output.push('|');
            }
        }
        output.push('\n');
        if i == 2 || i == 5 {
            output.push_str("---+---+---\n");
        }
    }

    output
}

pub fn deserialize(source: &str) -> [[usize; 9]; 9] {
    let mut grid = [[0usize; 9]; 9];

    let puzzle_lines = source
        .lines()
        .filter(|line| !line.starts_with('-')); // Skip separator lines

    for (row_idx, line) in puzzle_lines.enumerate().take(9) {
        let mut col_idx = 0;

        for ch in line.chars() {
            if col_idx >= 9 {
                break;
            }

            match ch {
                '1'..='9' => {
                    grid[row_idx][col_idx] = ch.to_digit(10).unwrap() as usize;
                    col_idx += 1;
                }
                '0' | ' ' => {
                    grid[row_idx][col_idx] = 0;
                    col_idx += 1;
                }
                _ => {} // Skip separators like |, +
            }
        }
        // Remaining columns stay 0 (already initialized)
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    static GRID: [[usize; 9]; 9] = [
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 7, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 8, 9],
    ];

    static STRING: &str = "\
100|000|000
020|000|000
003|000|000
---+---+---
000|000|000
000|000|000
000|000|000
---+---+---
000|006|000
000|000|700
000|000|089
";

    #[test]
    fn grid_to_string() {
        assert_eq!(serialize(&GRID), STRING);
    }

    #[test]
    fn string_to_grid() {
        assert_eq!(deserialize(STRING), GRID);
    }

    #[test]
    fn newline_ends_row() {
        // Trailing zeros can be omitted
        let short = "1\n\n\n\n\n\n\n\n";
        let grid = deserialize(short);
        assert_eq!(grid[0][0], 1);
        assert_eq!(grid[0][1], 0);
        assert_eq!(grid[1][0], 0);
    }

    #[test]
    fn spaces_are_blanks() {
        let with_spaces = "1 3|4 6|7 9";
        let grid = deserialize(with_spaces);
        assert_eq!(grid[0], [1, 0, 3, 4, 0, 6, 7, 0, 9]);
    }

    #[test]
    fn minimal_format() {
        // Most compact valid input
        let minimal = "\
123456789
456789123
789123456
234567891
567891234
891234567
345678912
678912345
912345678";
        let grid = deserialize(minimal);
        assert_eq!(grid[0], [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(grid[8], [9, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}