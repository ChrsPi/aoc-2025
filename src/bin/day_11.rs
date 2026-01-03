use std::collections::{HashMap};
#[derive(Debug, Clone)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

fn main() {
    println!("Day 11");

    let data = aoc2025::read_lines("data/day_11.txt");

    let mut devices: Vec<Device> = Vec::new();
    let start = Device {
        name: "".to_string(),
        outputs: Vec::new(),
    };
    devices.push(Device {
        name: "out".to_string(),
        outputs: Vec::new(),
    });

    // part 1
    // let count = calculate(data, devices, start, "you".to_string(), "out".to_string());
    // println!("Goal reached {count} times");

    // part_2
    let count_1 = calculate(
        data.clone(),
        devices.clone(),
        start.clone(),
        "svr".to_string(),
        "fft".to_string(),
    );
    let count_2 = calculate(
        data.clone(),
        devices.clone(),
        start.clone(),
        "fft".to_string(),
        "dac".to_string(),
    );
    let count_3 = calculate(
        data.clone(),
        devices.clone(),
        start.clone(),
        "dac".to_string(),
        "out".to_string(),
    );

    let count_4 = calculate(
        data.clone(),
        devices.clone(),
        start.clone(),
        "svr".to_string(),
        "dac".to_string(),
    );
    let count_5 = calculate(
        data.clone(),
        devices.clone(),
        start.clone(),
        "dac".to_string(),
        "fft".to_string(),
    );
    let count_6 = calculate(
        data.clone(),
        devices.clone(),
        start.clone(),
        "fft".to_string(),
        "out".to_string(),
    );

    println!(
        "Goal reached: {} times",
        count_1 * count_2 * count_3 + count_4 * count_5 * count_6
    );
}

fn calculate(
    data: Vec<String>,
    mut devices: Vec<Device>,
    mut start: Device,
    start_node: String,
    end_node: String,
) -> i64 {
    for line in data {
        println!("{:?}", line);

        let (head, tail) = line.split_once(":").unwrap();
        let outputs: Vec<String> = tail.trim().split(" ").map(|s| s.to_string()).collect();

        if head == start_node {
            start = Device {
                name: head.to_string(),
                outputs,
            };
        } else {
            devices.push(Device {
                name: head.to_string(),
                outputs,
            });
        }
    }

    println!("Start: {:?}", start);
    println!("{:?}", devices);

    let mut count_goal = 0;

    count_goal += traverse(start, end_node, Vec::new(), &mut HashMap::new(), devices);
    count_goal
}

fn traverse(
    device: Device,
    end: String,
    mut visited: Vec<String>,
    scores: &mut HashMap<String, i64>,
    devices: Vec<Device>,
) -> i64 {
    if device.name == end {
        return 1;
    }
    if device.name == "out" || visited.contains(&device.name) {
        return 0;
    }
    if scores.contains_key(&device.name) {
        return scores[&device.name];
    }

    visited.push(device.name.clone());
    let mut total = 0;
    for dev in device.outputs {
        let next = devices.iter().find(|d| d.name == dev).unwrap().clone();
        total += traverse(next, end.clone(), visited.clone(), scores, devices.clone());
    }

    visited.pop();
    scores.insert(device.name, total);
    total
}
