use std::env;
use std::fs;
use std::io::Write;

mod generators;
mod loader;
mod validators;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "generate" => generate(),
        "solve" => solve(),
        _ => print_usage(),
    }
}

fn generate() {
    let mut grid = generators::generate_grid();
    generators::remove_numbers(&mut grid);

    let output = loader::serialize(&grid);

    let mut file = fs::File::create("puzzle.txt").expect("Failed to create puzzle.txt");
    file.write_all(output.as_bytes()).expect("Failed to write puzzle.txt");

    println!("Generated puzzle.txt:\n{}", output);
}

fn solve() {
    let source = fs::read_to_string("puzzle.txt").expect("Failed to read puzzle.txt");
    let mut grid = loader::deserialize(&source);

    if validators::solve(&mut grid) {
        let output = loader::serialize(&grid);

        let mut file = fs::File::create("solution.txt").expect("Failed to create solution.txt");
        file.write_all(output.as_bytes()).expect("Failed to write solution.txt");

        println!("Solved! Written to solution.txt:\n{}", output);
    } else {
        eprintln!("No solution found. The puzzle may be invalid.");
    }
}

fn print_usage() {
    eprintln!("Usage: cargo generate | cargo solve");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  generate  Create a new puzzle in puzzle.txt");
    eprintln!("  solve     Solve puzzle.txt and write to solution.txt");
}
