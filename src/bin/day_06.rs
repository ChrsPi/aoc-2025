fn main() {
    let data = aoc2025::read_lines("data/day_06.txt");
    println!("Day 6");

    // part_1(data);
    part_2(data);
}

fn part_2(data: Vec<String>) {
    // left rotate number strings, then just p1

    let mut number_strings: Vec<String> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    let mut max_len = 0;

    for line in data {
        if line.contains('+') {
            for x in line.split_whitespace() {
                let c: char = x.parse().unwrap();
                ops.push(c);
            }
        } else {
            println!("Line: {:?}", line);
            if line.len() > max_len {
                max_len = line.len();
            }
            number_strings.push(line);
        }
    }

    let mut new_num_strings: Vec<String> = Vec::new();

    for i in 0..max_len {
        let mut new_line = String::with_capacity(max_len);
        for num_str in &number_strings {
            if i >= num_str.len() {
                new_line.push(' ');
            } else {
                new_line.push(num_str.chars().nth(i).unwrap());
            }
        }
        println!("{:?}", new_line);
        new_num_strings.push(new_line);
    }

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut num_line: Vec<u64> = Vec::new();

    for line in new_num_strings {
        if line.trim().is_empty() {
            println!("Num line: {:?}", num_line);
            numbers.push(num_line.clone());
            num_line.clear();
        } else {
            println!(
                "Non empty line: {:?}, trim: {:?}, {:?}",
                line,
                line.trim(),
                num_line
            );
            num_line.push(line.trim().parse::<u64>().unwrap());
        }
    }

    if !num_line.is_empty() {
        numbers.push(num_line);
    }

    for nums in &numbers {
        println!("{:?}", nums);
    }

    let mut answers: Vec<u64> = Vec::new();
    for (i, op) in ops.iter().enumerate() {
        let mut acc = 0;
        if *op == '*' {
            acc = 1;
        }

        for num in &numbers[i] {
           if *op == '+' {
               acc += num;
           } else {
               acc *= num;
           }
        }
        answers.push(acc);
    }

    println!("Part 2: {}", answers.iter().sum::<u64>());
}

fn part_1(data: Vec<String>) {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    for line in data {
        if line.contains('+') {
            for x in line.split_whitespace() {
                let c: char = x.parse().unwrap();
                ops.push(c);
            }
        } else {
            let mut num_line: Vec<u64> = Vec::new();
            for num in line.split_whitespace() {
                num_line.push(num.parse().unwrap());
            }
            numbers.push(num_line);
        }
    }

    let answers = calc_answers(&mut numbers, &mut ops);
    // println!("{:?}", answers);
    println!("Part 1: {}", answers.iter().sum::<u64>());
}

fn calc_answers(numbers: &mut Vec<Vec<u64>>, ops: &mut Vec<char>) -> Vec<u64> {
    let mut answers: Vec<u64> = Vec::new();
    for i in 0..ops.len() {
        let mut acc = 0;
        if ops[i] == '*' {
            acc = 1;
        }
        for num_line in &mut *numbers {
            if ops[i] == '+' {
                acc += num_line[i];
            } else if ops[i] == '*' {
                acc *= num_line[i];
            }
        }
        answers.push(acc);
    }
    answers
}
