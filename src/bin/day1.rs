fn main() {
    let lines = aoc2025::read_lines("data/day1.txt");

    println!("Day 1:");
    println!("Read {} lines", lines.len());

    for (i, line) in lines.iter().enumerate() {
        println!("Line {}: {}", i + 1, line);
    }
}