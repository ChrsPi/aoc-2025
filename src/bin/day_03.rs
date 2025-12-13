fn main() {
    let data = aoc2025::read_lines("data/day_03.txt");
    println!("Day 3");
    let banks: Vec<Vec<u64>> = data
        .iter()
        .map(|s| {
            let mut battery: Vec<u64> = Vec::new();
            for c in s.chars() {
                battery.push(c.to_digit(10).unwrap() as u64);
            }
            battery
        })
        .collect();

    part_2(banks);
}

#[derive(Copy, Clone)]
struct BatRank {
    i: usize,
    val: u64,
}

impl BatRank {
    fn reset(&mut self) {
        self.i = 0;
        self.val = 0;
    }
}
  
fn part_2(banks: Vec<Vec<u64>>) {
    let mut total_joltage: u64 = 0;
    for bank in banks {
        println!("{:?}", bank);
        let len = bank.len();
        let mut ranks = [BatRank { i: 0, val: 0 }; 12];
        let mut reset_rest = false;

        for (i, val) in bank.iter().enumerate() {
            println!("Loop: {}, val: {}, bank {:?}", i, val, bank);
            for (j, rank) in ranks.iter_mut().enumerate() {
                println!(
                    "Inner Loop j: {}, rank: i {}, val {}, reset: {}, len - i: {}",
                    j,
                    rank.i,
                    rank.val,
                    reset_rest,
                    len - i
                );
                if reset_rest {
                    rank.reset();
                } else {
                    if 12 - j <= len - i {
                        if *val > rank.val {
                            rank.i = j;
                            rank.val = *val;
                            reset_rest = true;
                        }
                    }
                }
            }
            reset_rest = false;
        }
        let mut joltage = 0;
        for (i, rank) in ranks.iter().enumerate() {
            println!("Rank i {}, val {} ", rank.i, rank.val);
            joltage += rank.val * 10_u64.pow(11 - i as u32);
        }

        total_joltage += joltage;

        println!("Current total: {total_joltage}");
    }
}

fn part_1(banks: Vec<Vec<u64>>) {
    let mut total_joltage: u64 = 0;
    for bank in banks {
        println!("{:?}", bank);
        let len = bank.len();
        let mut max_1: (usize, u64) = (0, 0);
        let mut max_2: (usize, u64) = (0, 0);

        for (i, val) in bank.iter().enumerate() {
            if i < len - 1 {
                if *val > max_1.1 {
                    max_1 = (i, *val);
                    max_2 = (0, 0);
                } else if *val > max_2.1 {
                    max_2 = (i, *val);
                }
            } else if *val > max_2.1 {
                max_2 = (i, *val);
            }
        }

        total_joltage += max_1.1 * 10 + max_2.1;
        println!(
            "Max 1: {}, Max 2: {}, current total: {}",
            max_1.1, max_2.1, total_joltage
        );
    }
}
