fn main() {
    println!("Day 7");
    let lines = aoc2025::read_lines("data/day_07.txt");
    let mut new_lines: Vec<Vec<char>> = Vec::new();
    let mut beam_timelines: Vec<Vec<u64>> = Vec::new();
    let mut count = 0;
    new_lines.push(lines[0].clone().chars().collect());
    beam_timelines.push(vec![1; new_lines[0].len()]);
    // println!("{:?}", new_lines[0]);
    for i in 1..lines.len() {
        let mut new_line: Vec<char> = lines[i].clone().chars().collect();
        // println!("{:?}", lines[i]);
        beam_timelines.push(vec![0; new_lines[0].len()]);
        for (j, c) in lines[i].chars().enumerate() {
            let above = new_lines[i - 1][j];

            if above == 'S' || above == '|' {
                if c == '.' {
                    new_line[j] = '|';
                    beam_timelines[i][j] += beam_timelines[i - 1][j]
                } else if c == '^' {
                    let mut is_split = false;
                    if j > 0 {
                        if new_line[j - 1] == '.' {
                            new_line[j - 1] = '|';
                            is_split = true;
                            beam_timelines[i][j - 1] = beam_timelines[i - 1][j]
                        } else if new_line[j - 1] == '|' {
                            beam_timelines[i][j - 1] += beam_timelines[i - 1][j];
                        }
                    }
                    if j < new_line.len() {
                        if new_line[j + 1] == '.' {
                            new_line[j + 1] = '|';
                            is_split = true;
                            beam_timelines[i][j + 1] = beam_timelines[i - 1][j]
                        } else if new_line[j + 1] == '|' {
                            beam_timelines[i][j + 1] += beam_timelines[i - 1][j];
                        }
                    }

                    if is_split {
                        count += 1;
                    }
                }
            }
        }
        println!("{:?}", String::from_iter(&new_line));
        new_lines.push(new_line);
    }
    println!("P1: Beam splits: {count}");
    println!("P2: Timelines: {}", beam_timelines[beam_timelines.len()-1].iter().sum::<u64>())

}
