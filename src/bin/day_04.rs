struct Step {
    i: i32,
    j: i32,
}

const ALL_STEPS: [Step; 8] = [
    Step { i: -1, j: -1 },
    Step { i: -1, j: 0 },
    Step { i: -1, j: 1 },
    Step { i: 0, j: -1 },
    // Step { i: 0, j: 0 }, // --> Don't count current pos
    Step { i: 0, j: 1 },
    Step { i: 1, j: -1 },
    Step { i: 1, j: 0 },
    Step { i: 1, j: 1 },
];

fn main() {
    let data = aoc2025::read_lines("data/day_04.txt");
    let mut grid: Vec<Vec<char>> = data.iter().map(|s| s.chars().collect()).collect();

    part_2(&mut grid);
}

fn part_2(grid: &mut Vec<Vec<char>>) {
    let mut count = 0;

    let mut removed = 1;

    while removed > 0 {
        removed = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '@' {
                    if count_neighbours(&grid, i as i32, j as i32) < 4 {
                        count += 1;
                        grid[i][j] = 'X';
                        removed += 1;
                    }
                }
            }
        }
    }
    println!("{count}");
}

fn part_1(grid: &Vec<Vec<char>>) {
    let mut count = 0;

    // for row in &grid {
    //     println!("{:?}", row);
    // }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                if count_neighbours(&grid, i as i32, j as i32) < 4 {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");
}

fn safe_steps(grid: &Vec<Vec<char>>, pos_i: i32, pos_j: i32) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    for step in ALL_STEPS {
        if step.i + pos_i >= 0
            && step.i + pos_i < grid.len() as i32
            && step.j + pos_j >= 0
            && step.j + pos_j < grid[0].len() as i32
        {
            steps.push(step);
        }
    }
    steps
}

fn count_neighbours(grid: &Vec<Vec<char>>, pos_i: i32, pos_j: i32) -> u32 {
    let mut count = 0;

    let steps = safe_steps(grid, pos_i, pos_j);

    for step in steps {
        let new_i = (pos_i + step.i) as usize;
        let new_j = (pos_j + step.j) as usize;
        if grid[new_i][new_j] == '@' {
            count += 1;
        }
    }

    count
}
