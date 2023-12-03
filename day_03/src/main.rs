use std::{fs::read_to_string, collections::{HashMap, HashSet}};

fn main() {
    let lines: Vec<String> = read_to_string("./input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    // println!("Part - 1 Res = {}", part1(lines));

    let lines: Vec<String> = read_to_string("./input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("Part - 2 Res = {}", part2(lines));
}

fn part2(lines: Vec<String>) -> u64 {
    let mut grid: Vec<Vec<char>> = Vec::new();

    lines.iter().for_each(|line| {
        let mut t: Vec<char> = Vec::new();
        line.chars().for_each(|ch| {
            t.push(ch);
        });

        grid.push(t);
    });

    solve_part_2(grid)
}

fn part1(lines: Vec<String>) -> u32 {
    let mut grid: Vec<Vec<char>> = Vec::new();

    lines.iter().for_each(|line| {
        let mut t: Vec<char> = Vec::new();
        line.chars().for_each(|ch| {
            t.push(ch);
        });

        grid.push(t);
    });

    solve_part_1(grid)
}

fn solve_part_2(grid: Vec<Vec<char>>) -> u64 {
    let mut total: u64 = 0;

    let R = grid.len();
    let C = grid.get(0).unwrap().len();
    let positions = vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,-1), (1, -1), (-1, 1)];

    let mut map: HashMap<(i32, i32), Vec<u64>> = HashMap::new();
    let mut gears: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..R {
        let mut number: u64 = 0;
        let mut found_special_symbol = false;
        gears.clear();

        for j in 0..C {
            let ch = grid.get(i).unwrap().get(j).unwrap();
            if ch.is_digit(10) {
                number = number * 10 + ch.to_digit(10).unwrap() as u64;

                positions.iter().for_each(|position| {
                    let rr: i32 = i as i32 + position.0 as i32;
                    let cc: i32 = j as i32 + position.1 as i32;

                    if 0 <= rr && rr < R as i32 && 0 <= cc && cc < C as i32 {

                        let ch2 = grid.get(rr as usize).unwrap().get(cc as usize).unwrap();

                        if ch2.eq(&'*') {
                            gears.insert((rr, cc));
                            found_special_symbol = true;
                        }
                    }
                });
            } else {
                if found_special_symbol && number > 0 {
                    for gear in gears.iter() {
                        map.entry(*gear).or_insert(Vec::new()).push(number);
                    }
                }

                found_special_symbol = false;
                number = 0;
                gears.clear();
            }
        }


        if found_special_symbol && number > 0 { // after the end of the line
            for gear in gears.iter() {
                map.entry(*gear).or_insert(Vec::new()).push(number);
            }
        }
    }
    
    let mut v: Vec<((i32, i32), Vec<u64>)> = map.clone().into_iter().collect();
    v.sort_by_key(|&(key, _)| key);

    for (key, value) in v.iter() {
        println!("{:?}: {:?}", key, value);
    }

    for (k, value) in map.into_iter() {
        if value.len() == 2 {
            total += value.get(0).unwrap() * value.get(1).unwrap();
        }
    }

    return total;
}


fn solve_part_1(grid: Vec<Vec<char>>) -> u32 {
    let mut total = 0;

    let R = grid.len();
    let C = grid.get(0).unwrap().len();
    let positions = vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,-1), (1, -1), (-1, 1)];

    let mut numbers: Vec<u32> = Vec::new();


    for i in 0..R {
        let mut number = 0;
        let mut found_special_symbol = false;
        let mut current_row: Vec<u32> = Vec::new();

        for j in 0..C {
            let ch = grid.get(i).unwrap().get(j).unwrap();
            if ch.is_digit(10) {
                number = number * 10 + ch.to_digit(10).unwrap();

                positions.iter().for_each(|position| {
                    let rr: i32 = i as i32 + position.0 as i32;
                    let cc: i32 = j as i32 + position.1 as i32;

                    if 0 <= rr && rr < R as i32 && 0 <= cc && cc < C as i32 {

                        let ch2 = grid.get(rr as usize).unwrap().get(cc as usize).unwrap();

                        if !ch2.eq(&'.') && !ch2.is_digit(10) {
                            found_special_symbol = true;
                        }
                    }
                });
            } else {
                if found_special_symbol {
                    current_row.push(number);
                    numbers.push(number);
                }

                found_special_symbol = false;
                number = 0;
            }
        }

        println!("Row - {}, Numbers - {:?}", i, current_row);


        if found_special_symbol {
            numbers.push(number);
        }
    }

    total = numbers.iter().sum();
    return total;
}
