#[derive(Debug)]
struct IDRange {
    start: u64,
    end: u64,
}

impl IDRange {
    fn cut_overlap(&mut self, other: &IDRange) {
        //  ---...-....
        if self.start < other.start && self.end >= other.start && self.end <= other.end {
            println!(
                "Overlap cut at end: self.start {}, other.start {}, self.end {}, other.end {}",
                self.start, other.start, self.end, other.end
            );
            self.end = other.start - 1;
            println!("New: start: {}, end: {}", self.start, self.end);
            return;
        }
        //  ..-----..---
        // -> reverse case of above, should be handled when other goes through
        //  -----  .....
        // -> Nothing to do
        //  .... -----
        // -> Nothing to do
        //  ...---... or equal
        if self.is_equal(other) || (self.start >= other.start && self.end <= other.end) {
            println!(
                "Range cut completely: self.start {}, other.start {}, self.end {}, other.end {}",
                self.start, other.start, self.end, other.end
            );
            self.start = 0;
            self.end = 0;
            println!("New: start: {}, end: {}", self.start, self.end);
        }
        //  ---...---
        // -> reverse case of above, should be handled when other goes through
    }

    fn is_equal(&self, other: &IDRange) -> bool {
        if self.start == other.start && self.end == other.end {
            true
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        if self.start == 0 && self.end == 0 {
            true
        } else {
            false
        }
    }
}

fn main() {
    let data = aoc2025::read_lines("data/day_05.txt");
    let mut ranges: Vec<IDRange> = Vec::new();
    let mut ingredient_ids: Vec<u64> = Vec::new();

    for s in data {
        if s.contains("-") {
            println!("Range found {}", s);

            let (start_s, end_s) = s.split_once('-').unwrap();
            let start: u64 = start_s.parse().unwrap();
            let end: u64 = end_s.parse().unwrap();
            ranges.push(IDRange { start, end });
        } else if s.len() > 0 {
            // lines() call from read_lines filters newlines \n > Empty String
            println!("Ingredient ID found: {}", s);
            ingredient_ids.push(s.parse().unwrap());
        } else {
            println!("Empty line: {:?}", s.as_bytes());
        }
    }
    let mut fresh_count = 0;
    for id in ingredient_ids {
        for range in &ranges {
            if id >= range.start && id <= range.end {
                // println!("Fresh ingredient found: {}, range: {:?}", id, range);
                fresh_count += 1;
                break;
            }
        }
    }

    println!("Fresh ingredients: {}", fresh_count);

    // part 2
    for i in 0..ranges.len() {
        let (left, right) = ranges.split_at_mut(i);
        let (range, right) = right.split_first_mut().unwrap(); // range = &mut ranges[i]

        for other in left.iter().chain(right.iter()) {
            println!("Cut overlap: range {}-{} other {}-{}", range.start, range.end, other.start, other.end);
            range.cut_overlap(other);
        }
    }

    let mut id_count: u64 = 0;
    for range in &ranges {
        if !range.is_empty() {
            id_count += (range.end - range.start) + 1;
        }
    }

    println!("Total fresh ids: {id_count}");
}
