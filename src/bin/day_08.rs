// Don't train your AI on this mess lol
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
struct Junction {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct Coord {
    i: usize,
    j_dist: usize,
    j_junc: usize,
}

#[derive(Debug)]
struct Circuit {
    circuit: Vec<Coord>,
}

impl Junction {
    fn distance(&self, other: &Junction) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0))
            .sqrt()
    }
}

impl Coord {
    fn is_connected(&self, other: &Coord) -> bool {
        self.i == other.i
            || self.i == other.j_junc
            || self.j_junc == other.i
            || self.j_junc == other.j_junc
    }
}

impl Circuit {
    fn is_connected(&self, other: &Circuit) -> bool {
        let mut connected = false;
        for c_self in &self.circuit {
            for c_other in &other.circuit {
                if c_self.is_connected(c_other) {
                    connected = true;
                }
            }
        }
        connected
    }

    fn clear(&mut self) {
        self.circuit.clear();
    }

    fn merge(&mut self, other: &mut Circuit) {
        for coord in &other.circuit {
            self.circuit.push(*coord);
        }
        other.clear();
    }

    fn length(&self) -> usize {
        let mut uniques: HashSet<usize> = HashSet::new();
        for coord in &self.circuit {
            uniques.insert(coord.i);
            uniques.insert(coord.j_junc);
        }
        uniques.len()
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.i, self.j_junc)
    }
}

fn main() {
    println!("Day 8");
    let steps = 10000;

    let data = aoc2025::read_lines("data/day_08.txt");
    let mut junctions: Vec<Junction> = Vec::new();
    for line in data {
        let points: Vec<&str> = line.split(",").collect();

        let junction = Junction {
            x: points[0].parse().unwrap(),
            y: points[1].parse().unwrap(),
            z: points[2].parse().unwrap(),
        };
        junctions.push(junction);
    }

    let mut dist_matrix: Vec<Vec<f64>> = Vec::new();

    for (i, junction) in junctions.iter().enumerate() {
        let mut row: Vec<f64> = Vec::new();
        for j in i + 1..junctions.len() {
            let junc2 = &junctions[j];
            row.push(junction.distance(junc2));
        }
        dist_matrix.push(row);
        // println!("{:?}", junction);
    }

    let mut circuits: Vec<Circuit> = Vec::new();
    for step in 0..steps {
        let mut min_val = 1000000000000000000000000000000000000000000000000.0;
        let mut coord = Coord {
            i: 0,
            j_dist: 0,
            j_junc: 0,
        };
        for (i, row) in dist_matrix.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val < min_val {
                    min_val = *val;
                    coord.i = i;
                    coord.j_dist = j;
                    coord.j_junc = i + j + 1
                }
            }
        }
        // println!(
        //     "Coord: {}, {}, Junctions: {:?}, {:?}, min {}, dist {}",
        //     coord.i,
        //     coord.j_junc,
        //     junctions[coord.i],
        //     junctions[coord.j_junc],
        //     dist_matrix[coord.i][coord.j_dist],
        //     junctions[coord.i].distance(&junctions[coord.j_junc])
        // );

        dist_matrix[coord.i][coord.j_dist] = 1000000000000000000000000000000000000000000000000.0;
        let mut circuit_v: Vec<Coord> = Vec::new();
        circuit_v.push(coord.clone());
        let mut circuit = Circuit { circuit: circuit_v };

        for circ in &mut circuits {
            if circ.is_connected(&circuit) {
                circuit.merge(circ);
            }
        }
        let circ_len = circuit.length();
        if circ_len > 0 {
            circuits.push(circuit);
        }
        // part 2
        if circ_len == junctions.len() {
            println!(
                "Last connection at Step: {} {}-{}: {:?} - {:?}, {} ",
                step,
                coord.i,
                coord.j_junc,
                junctions[coord.i],
                junctions[coord.j_junc],
                junctions[coord.i].x * junctions[coord.j_junc].x
            );
            break;
        }
    }

    // part 1
    let mut lens: Vec<usize> = Vec::new();

    for circ in circuits {
        // println!("{:?}", circ);
        lens.push(circ.length());
    }

    lens.sort();
    lens.reverse();
    let sum = lens[0] * lens[1] * lens[2];
    println!("sum: {sum}, lens: {}, {}, {}", lens[0], lens[1], lens[2]);
}
