use std::{fs::read_to_string, collections::HashSet, cmp};

fn main() {
    let lines: Vec<String> = read_to_string("./src/input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("{}", part2(lines.
                         into_iter().
                         map(|line| line.chars().collect()).
                         collect()
                        ));
}


fn part2(grid: Vec<Vec<char>>) -> u64 {
    let R: usize = grid.len() as usize;
    let C: usize = grid.get(0).unwrap().len() as usize;

    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut rows_containing_galaxy: HashSet<usize> = HashSet::new();
    let mut cols_containing_galaxy: HashSet<usize> = HashSet::new();

    for i in 0..R {
        for j in 0..C {
            let v: &char = grid.get(i).unwrap().get(j).unwrap();

            if *v == '#' {
                points.push((i, j));

                rows_containing_galaxy.insert(i);
                cols_containing_galaxy.insert(j);
            }
        }
    }

    let mut total = 0;

    for (i, point) in points.iter().enumerate() {
        for (_, another_point) in points[0..i].iter().enumerate() {
            let r1 = point.0;
            let r2 = another_point.0;

            let c1 = point.1;
            let c2 = another_point.1;

            for r in cmp::min(r1, r2)..cmp::max(r1, r2) {
                if !rows_containing_galaxy.contains(&r) {
                    total += 1000000; // twice as big, not galaxy
                } else {
                    total += 1;
                }
            }

            for c in cmp::min(c1, c2)..cmp::max(c1, c2) {
                if !cols_containing_galaxy.contains(&c) {
                    total += 1000000; // twice as big, not galaxy
                } else {
                    total += 1;
                }
            }
        }
    }

    return total;
}

fn part1(grid: Vec<Vec<char>>) -> u64 {
    let R: usize = grid.len() as usize;
    let C: usize = grid.get(0).unwrap().len() as usize;

    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut rows_containing_galaxy: HashSet<usize> = HashSet::new();
    let mut cols_containing_galaxy: HashSet<usize> = HashSet::new();

    for i in 0..R {
        for j in 0..C {
            let v: &char = grid.get(i).unwrap().get(j).unwrap();

            if *v == '#' {
                points.push((i, j));

                rows_containing_galaxy.insert(i);
                cols_containing_galaxy.insert(j);
            }
        }
    }

    let mut total = 0;

    for (i, point) in points.iter().enumerate() {
        for (_, another_point) in points[0..i].iter().enumerate() {
            let r1 = point.0;
            let r2 = another_point.0;

            let c1 = point.1;
            let c2 = another_point.1;

            for r in cmp::min(r1, r2)..cmp::max(r1, r2) {
                if !rows_containing_galaxy.contains(&r) {
                    total += 2; // twice as big, not galaxy
                } else {
                    total += 1;
                }
            }

            for c in cmp::min(c1, c2)..cmp::max(c1, c2) {
                if !cols_containing_galaxy.contains(&c) {
                    total += 2; // twice as big, not galaxy
                } else {
                    total += 1;
                }
            }
        }
    }

    return total;
}
