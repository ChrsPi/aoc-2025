#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn rect_area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn cross_product(a: &Point, b: &Point, p: &Point) -> i64 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

fn on_boundary(a: &Point, b: &Point, p: &Point) -> bool {
    if cross_product(a, b, p) != 0 {
        return false;
    }

    let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
    let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
    p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
}

fn line_intersects(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
    let o1 = cross_product(a, b, c);
    let o2 = cross_product(a, b, d);
    let o3 = cross_product(c, d, a);
    let o4 = cross_product(c, d, b);

    if o1 == 0 && on_boundary(a, b, c) {
        return false;
    }
    if o2 == 0 && on_boundary(a, b, d) {
        return false;
    }
    if o3 == 0 && on_boundary(c, d, a) {
        return false;
    }
    if o4 == 0 && on_boundary(c, d, b) {
        return false;
    }

    (o1 > 0) != (o2 > 0) && (o3 > 0) != (o4 > 0)
}

fn pip(points: &Vec<Point>, p: &Point) -> bool {
    for i in 0..points.len() {
        let a = &points[i];
        let b = &points[(i + 1) % points.len()];

        if on_boundary(a, b, p) {
            return true;
        }
    }

    let mut inside = false;

    for i in 0..points.len() {
        let a = &points[i];
        let b = &points[(i + 1) % points.len()];

        // raycast to right -> is AB strictly above/below p horizontally?
        if (a.y > p.y) == (b.y > p.y) {
            continue;
        }

        let dy = b.y - a.y;
        let lhs = (p.x - a.x) * dy;
        let rhs = (p.y - a.y) * (b.x - a.x);

        if (dy > 0 && lhs < rhs) || (dy <= 0 && lhs > rhs) {
            inside = !inside;
        }
    }

    inside
}

fn main() {
    println!("Day 9");

    let data = aoc2025::read_lines("data/day_09.txt");
    let mut points: Vec<Point> = Vec::new();
    for line in data {
        let (x_s, y_s) = line.split_once(",").unwrap();
        let x = x_s.parse::<i64>().unwrap();
        let y = y_s.parse::<i64>().unwrap();
        // println!("{},{}", x, y);
        points.push(Point { x, y });
    }

    println!(
        "1: {:?}, 2: {:?}: {}",
        points[0],
        points[1],
        points[0].rect_area(&points[1])
    );

    let mut max_area = 0;
    let mut max_points = (&points[0], &points[0]);
    for (i, p1) in points.iter().enumerate() {
        for j in i + 1..points.len() {
            let area = p1.rect_area(&points[j]);
            if area > max_area {
                max_area = area;
                max_points = (p1, &points[j]);
            }
        }
    }

    println!(
        "Max area: {}, from: {:?}, {:?}",
        max_area, max_points.0, max_points.1
    );

    // part 2. Instead of comparing everything, could also take sorted list from 1
    let mut max_area = 0;
    let mut max_points = (&points[0], &points[0]);
    for (i, p1) in points.iter().enumerate() {
        for j in i + 1..points.len() {
            let corner_points = vec![
                Point { x: p1.x, y: p1.y },
                Point {
                    x: p1.x,
                    y: points[j].y,
                },
                Point {
                    x: points[j].x,
                    y: points[j].y,
                },
                Point {
                    x: points[j].x,
                    y: p1.y,
                },
            ];

            // filter out obvious ones, pip was my first solution so why not use it :D
            if corner_points.iter().all(|p| pip(&points, p)) {
                let mut inside = true;

                for k in 0..corner_points.len() {
                    let a = &corner_points[k];
                    let b = &corner_points[(k + 1) % corner_points.len()];

                    for l in 0..points.len() {
                        let pa = &points[l];
                        let pb = &points[(l + 1) % points.len()];
                        if line_intersects(pa, pb, a, b) {
                            inside = false;
                            break;
                        }
                    }
                }

                if !inside {
                    continue;
                }

                let area = p1.rect_area(&points[j]);
                if area > max_area {
                    max_area = area;
                    max_points = (p1, &points[j]);
                }
            }
        }
    }

    println!(
        "Max area p2: {}, from: {:?}, {:?}",
        max_area, max_points.0, max_points.1
    );
}
