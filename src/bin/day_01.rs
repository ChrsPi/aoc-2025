struct Rotation {
    to_left: bool,
    clicks: i32,
}

fn main() {
    let lines = aoc2025::read_lines("data/day_01.txt");
    let mut rotations: Vec<Rotation> = Vec::new();

    println!("Day 1:");

    for line in lines.iter() {
        let to_left = line.starts_with('L');
        let clicks: i32 = *&line[1..].parse().unwrap();

        rotations.push(Rotation { to_left, clicks })
    }

    // part1(&mut rotations);
    part2(&mut rotations);
}

fn part2(rotations: &mut Vec<Rotation>) {
    let mut pos = 50;
    let mut counter = 0;
    for rot in rotations.iter() {
        println!(
            "to_left: {:<5}, clicks: {:<3}, current_pos: {:<2}",
            rot.to_left, rot.clicks, pos
        );

        let mut past_zero = rot.clicks.div_euclid(100);
        let clicks_left = rot.clicks % 100;
        if (past_zero > 0) {
            println!("Div euclid: {past_zero}");
        }
        if rot.to_left {
            if clicks_left > pos {
                if pos != 0 && pos != 100 {
                    println!("Past Zero!");
                    past_zero += 1;
                }
                pos = 100 - ((pos - clicks_left).abs() % 100);
            } else {
                pos = pos - clicks_left;
            }
        } else {
            if pos + clicks_left > 100 {
                println!("Past Zero!");
                past_zero += 1;
            }
            pos = (pos + clicks_left) % 100;
        }
        if pos == 0 || pos == 100 {
            counter += 1;
        }
        counter += past_zero;
    }

    println!("Counts: {}", counter)
}

fn part1(rotations: &mut Vec<Rotation>) {
    let mut pos = 50;
    let mut counter = 0;
    for rot in rotations.iter() {
        println!(
            "to_left: {:<5}, clicks: {:<3}, current_pos: {:<2}",
            rot.to_left, rot.clicks, pos
        );

        if rot.to_left {
            if rot.clicks > pos {
                pos = 100 - ((pos - rot.clicks).abs() % 100);
            } else {
                pos = pos - rot.clicks;
            }
        } else {
            pos = (pos + rot.clicks) % 100;
        }
        if pos == 0 || pos == 100 {
            counter += 1;
        }
    }

    println!("Counts: {}", counter)
}
