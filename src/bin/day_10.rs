use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};
use regex::Regex;
#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

#[derive(Debug)]
struct Node {
    lights: Vec<bool>,
    depth: u32,
}

fn solve(joltages: &[u32], buttons: &[Vec<usize>]) -> Vec<u64> {
    let m = joltages.len();
    let n = buttons.len();

    let mut vars = variables!();
    let mut x = Vec::with_capacity(n);

    // upper bound: no button needs to be pressed more than max goal
    let ub = *joltages.iter().max().unwrap_or(&0);

    for _ in 0..n {
        x.push(vars.add(variable().integer().min(0).max(ub)));
    }

    // minimize total presses
    let mut model = vars
        .minimise(x.iter().copied().sum::<Expression>())
        .using(default_solver);

    // constraints: for each i, sum_{j: i in S_j} x_j == goals[i]
    for i in 0..m {
        let mut expr: Expression = 0.into();
        for (j, idxs) in buttons.iter().enumerate() {
            if idxs.contains(&i) {
                expr = expr + x[j];
            }
        }
        model = model.with(constraint!(expr == joltages[i]));
    }

    let sol = model.solve().unwrap();
    x.into_iter().map(|v| sol.value(v).round() as u64).collect()
}

fn take_step(curr_lights: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
    let mut new_ligths = curr_lights.clone();
    for but in button {
        new_ligths[*but] = !new_ligths[*but];
    }
    new_ligths
}

fn main() {
    println!("Day 10");

    let data = aoc2025::read_lines("data/day_10.txt");

    // using regex just for practice
    let re_lights = Regex::new(r"\[([.#]+)]").unwrap();
    let re_buttons = Regex::new(r"\((\d+(?:,\d+)*)\)").unwrap();
    let re_joltages = Regex::new(r"\{(\d+(?:,\d+)*)}").unwrap();

    let mut machines: Vec<Machine> = Vec::new();

    for line in data {
        println!("{}", line);
        let caps_l = re_lights
            .captures(&line)
            .and_then(|c| {
                c.get(1).map(|m| {
                    m.as_str()
                        .chars()
                        .map(|ch| if ch == '#' { true } else { false })
                        .collect()
                })
            })
            .unwrap();

        let caps_b: Vec<Vec<usize>> = re_buttons
            .captures_iter(&line)
            .map(|c| {
                c.get(1)
                    .map(|m| {
                        m.as_str()
                            .split(",")
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect()
                    })
                    .unwrap()
            })
            .collect();

        let caps_j: Vec<u32> = re_joltages
            .captures(&line)
            .and_then(|c| {
                c.get(1).map(|m| {
                    m.as_str()
                        .split(",")
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect()
                })
            })
            .unwrap();
        let machine = Machine {
            lights: caps_l,
            buttons: caps_b,
            joltage: caps_j,
        };
        println!("{:?}", machine);
        machines.push(machine);
    }

    // plain old bfs
    let mut presses = 0;
    for machine in &machines {
        let root = Node {
            lights: machine.lights.iter().map(|_| false).collect(),
            depth: 0,
        };
        let mut curr_nodes: Vec<Node> = Vec::new();
        curr_nodes.push(root);
        'outer: loop {
            let mut new_nodes: Vec<Node> = Vec::new();
            for node in &curr_nodes {
                for but in &machine.buttons {
                    let new_lights = take_step(&node.lights, &but);
                    if new_lights == machine.lights {
                        presses += node.depth + 1;
                        println!(
                            "Final step from {:?} with {:?} to {:?} after {} steps",
                            node,
                            but,
                            new_lights,
                            node.depth + 1
                        );
                        break 'outer;
                    } else {
                        new_nodes.push(Node {
                            lights: new_lights,
                            depth: node.depth + 1,
                        })
                    }
                }
            }
            curr_nodes = new_nodes;
        }
    }
    println!("Total steps for activating: {presses}");

    // Part 2 - using library for ILP
    println!("Part 2");

    let mut presses = 0;

    for machine in machines {
        let res = solve(&machine.joltage, &machine.buttons);
        presses += res.iter().sum::<u64>();
    }
    println!("Total steps for joltage: {presses}");
}
