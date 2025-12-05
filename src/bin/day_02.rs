struct IDRange {
    start: u128,
    end: u128,
}

fn main() {
    let data = aoc2025::read_input("data/day_02.txt");

    println!("Day 2");
    // println!("Data: {}", data);

    let ranges: Vec<IDRange> = data
        .split(',')
        .map(|s| {
            let (start_s, end_s) = s.split_once('-').unwrap();
            // println!("s: {}, start_s {}, end_s {}", s, start_s, end_s);
            let start: u128 = start_s.parse().unwrap();
            let end: u128 = end_s.parse().unwrap();
            // println!("s: {}, start_s {}, end_s {}, start {}, end {}", s, start_s, end_s, start, end);
            IDRange { start, end }
        })
        .collect();

    part_2(ranges);
}

fn part_2(ranges: Vec<IDRange>) {
    let mut counter: u128 = 0;
    for id_range in ranges {
        // println!("Range: {}-{}", id_range.start, id_range.end);
        for i in id_range.start..=id_range.end {
            let s = i.to_string();
            let len = s.len();
            // max length of strings seems to be around 10, so only need to check for repeated patterns of length 2, 3, 5 and 7

            if len % 2 == 0 {
                let (first, second) = s.split_at(len / 2);
                // println!("{}", s);
                // println!("First {}, second {}", first, second);
                if first == second {
                    // println!("{}", s);
                    // println!("Hit 2!");
                    counter += i;
                    continue;
                }
            }
            if len % 3 == 0 {
                let (first, splits) = s.split_at(len / 3);
                let (second, third) = splits.split_at(len / 3);
                if first == second && first == third {
                    // println!("{}", s);
                    // println!("Hit 3!");
                    counter += i;
                    continue;
                }
            }
            if len % 5 == 0 {
                let (first, splits) = s.split_at(len / 5);
                let (second, splits) = splits.split_at(len / 5);
                let (third, splits) = splits.split_at(len / 5);
                let (fourth, fifth) = splits.split_at(len / 5);

                if first == second && first == third && first == fourth && first == fifth {
                    println!("{}", s);
                    println!("Hit 5!");
                    counter += i;
                    continue;
                }
            } else if len % 7 == 0 {
                // kinda unnecessary, since at this point all digits will be the same
                // println!("7: {}", s);
                let (first, splits) = s.split_at(len / 7);
                let (second, splits) = splits.split_at(len / 7);
                let (third, splits) = splits.split_at(len / 7);
                let (fourth, splits) = splits.split_at(len / 7);
                let (fifth, splits) = splits.split_at(len / 7);
                let (sixth, seventh) = splits.split_at(len / 7);

                // println!("1 {}, 2 {}, 3 {}, 4 {}, 5 {}, 6 {}, 7 {}", first, second, third, fourth, fifth, sixth, seventh);

                if first == second
                    && first == third
                    && first == fourth
                    && first == fifth
                    && first == sixth
                    && first == seventh
                {
                    println!("{}", s);
                    println!("Hit 7!");
                    counter += i;
                }
            }
        }
    }
    println!("Sum of invalid IDs: {}", counter);
}

fn part_1(ranges: Vec<IDRange>) {
    let mut counter: u128 = 0;
    for id_range in ranges {
        // println!("Range: {}-{}", id_range.start, id_range.end);
        for i in id_range.start..=id_range.end {
            let s = i.to_string();
            let len = s.len();
            if len % 2 != 0 {
                continue;
            } else {
                let (first, second) = s.split_at(len / 2);
                // println!("{}", s);
                // println!("First {}, second {}", first, second);
                if first == second {
                    // println!("Hit!");
                    counter += i;
                }
            }
        }
    }
    println!("Sum of invalid IDs: {}", counter);
}
