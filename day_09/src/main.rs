use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("./src/input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    println!("{}", part2(lines));
}

fn part2(lines: Vec<String>) -> i32 {
    let mut total = 0;
    lines.iter().for_each(|line| {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|e| { e.parse().unwrap() })
            .collect();

        let prev = get_prev(&nums);
        total += prev;
    });

    return total;
}

fn get_prev(nums: &Vec<i32>) -> i32 {
    if nums.iter().all(|&x| x == 0) {
        return 0;
    }

    let mut nxt_iteration = Vec::new();

    for i in 1..nums.len() {
        let diff = nums.get(i).unwrap() - nums.get(i - 1).unwrap();
        nxt_iteration.push(diff);
    }

    let prev = get_prev(&nxt_iteration);

    let first = nums.get(0).unwrap();

    // println!("{:?} - Prev = {} with first = {} , prev = {} ", nums, first - prev, first, prev);

    return first - prev;
}

fn part1(lines: Vec<String>) -> i32 {
    let mut total = 0;
    lines.iter().for_each(|line| {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|e| { e.parse().unwrap() })
            .collect();

        // dbg!(&nums);

        let nxt = get_next(&nums);
        // dbg!(nxt);
        total += nxt;
    });

    return total;
}

fn get_next(nums: &Vec<i32>) -> i32 {

    if nums.iter().all(|&x| x == 0) {
        // println!("{:?} - {}", nums, 0);
        return 0;
    }

    let mut nxt_iteration = Vec::new();

    for i in 1..nums.len() {
        let diff = nums.get(i).unwrap() - nums.get(i - 1).unwrap();
        nxt_iteration.push(diff);
    }

    let nxt = get_next(&nxt_iteration);

    let last = nums.get(nums.len() - 1).unwrap();

    // println!("{} + {} = {}", last, nxt, last + nxt);

    return last + nxt;
}
