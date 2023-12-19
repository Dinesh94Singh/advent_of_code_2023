use std::{fs::read_to_string, collections::HashMap, collections::{VecDeque, HashSet}};
use gcd::Gcd;

fn main() {
    let lines: Vec<String> = read_to_string("./src/input3.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("{}", part2(lines));
}

fn build_map(lines: &Vec<String>) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut start: Option<String> = None;
    lines[2..].iter().for_each(|line| {
        let parts: Vec<&str> = line.split("=").collect();
        let source = parts.get(0).unwrap().trim();

        if start.is_none() {
            start = Some(source.to_string());
        }


        let mut dests: &str = parts.get(1).unwrap().trim();
        dests = dests.trim_matches(|c| c == '(' || c == ')');

        let dest_parts: Vec<&str> = dests.split(", ").collect();

        let left_part = dest_parts.get(0).unwrap();
        let right_part = dest_parts.get(1).unwrap();

        map.entry(source.to_string()).
            or_insert((left_part.to_string(), right_part.to_string()));
    });

    return map;
}

fn part2(lines: Vec<String>) -> u128 {
    let map = build_map(&lines);
    let directions: Vec<char> = lines.get(0).unwrap().chars().collect();

    // -------
    let mut keys = Vec::new();

    map.keys().into_iter().for_each(|key| {
        if key.ends_with('A') {
            keys.push(key);
        }
    });

    dbg!(&map);
    dbg!(&keys);

    let mut cycles: Vec<Vec<u64>> = Vec::new();
    // cycles is a 2d array
    // 0th idx element is until the first z is encountered
    // 1st idx element is until the same word is encountered - cycle length

    keys.into_iter().for_each(|key| {
        let mut cycle: Vec<u64> = Vec::new();
        let mut first: Option<String> = None;
        let mut step_count = 0;
        let mut idx = 0;


        let mut curr = key.to_string();
        loop {

            println!("{:?}", curr);

            while step_count == 0 || !curr.ends_with('Z') {
                step_count += 1;

                if idx == directions.len() {
                    idx = 0;
                }

                let direction = *directions.get(idx).unwrap();
                let neis: &(String, String) = map.get(&curr).unwrap();

                if direction == 'L' {
                    curr = (*neis.0).to_string();
                } else {
                    curr = (*neis.1).to_string();
                }

                println!(" -> {:?}", curr);

                idx += 1;
            }


            cycle.push(step_count);

            if first.is_none() {
                println!("Found first word which ends with z for {} with {}", key, curr);
                first = Some(curr.clone());
                // Clone is important here -
                // borrow of moved value: `curr` happens when we do Some(curr)
                // line: 72 complains without cloning
                step_count = 0;
            } else if first.is_some() {
                println!("Found second word which ends with z for {} with {}", key, curr);
                break;
            }
        }

        cycles.push(cycle);
        println!("\n\n");

    });

    dbg!(&cycles);

    let mut lcm = *cycles.get(0).unwrap().get(0).unwrap() as u128;

    // (n1 * n2) / gcd = lcm

    cycles[1..].iter().for_each(|cycle| {
        let n: u128 = *cycle.get(0).unwrap() as u128;
        let hcf = lcm.gcd(n); // get HCF (i.e., GCD) of these two numbers number
        lcm = (lcm / hcf) * n;
    });

    return lcm;
}

fn part2_tle(lines: Vec<String>) -> i32 {
    let map = build_map(&lines);
    let directions: Vec<char> = lines.get(0).unwrap().chars().collect();
    // dbg!(&map);

    let mut deque: VecDeque<String> = VecDeque::new();

    let keys = map.keys();

    keys.into_iter().for_each(|key| {
        if key.ends_with('A') {
            deque.push_back(key.to_string());
        }
    });

    let mut k = 0;
    let mut direction = 'R';
    let mut seen: HashSet<String> = HashSet::new();

    let mut steps = 0;
    while !deque.len() > 0  {
        if k == directions.len() {
            println!("Repeating");
            k = 0;
        }

        direction = *directions.get(k).unwrap();

        let mut t = deque.len() as i32;
        let mut meets_criteria = true;


        let mut key = format!("{}#{}#",direction,k);
        while t > 0 && deque.len() > 0 {
            let src = deque.pop_front().unwrap();

            key.push_str(&src);
            key.push_str("#");

            if !src.ends_with('Z') {
                meets_criteria = false;
            }

            map.contains_key(&src);

            let neis: &(String, String) = map.get(&src).unwrap();

            if direction == 'L' {
                deque.push_back(neis.0.to_string());
            } else {
                deque.push_back(neis.1.to_string());
            }

            t -= 1;
        }

        if seen.contains(&key) {
            println!("Key Already exists, Looping");
            return -1;
        }

        seen.insert(key);

        if meets_criteria {
            println!("Met Criteria");
            return steps as i32;
        }


        k += 1;
        steps += 1;
    }


    return steps as i32;
}

fn part1(lines: Vec<String>) -> i32 {
    let directions: Vec<char> = lines.get(0).unwrap().chars().collect();
    let mut map: HashMap<String, (String, String)> = build_map(&lines);
    let mut deque: VecDeque<String> = VecDeque::new();
    let mut start: Option<String> = None;

    if let Some(s) = start {
        deque.push_back("AAA".to_string());
    }

    let mut k = 0;
    let mut direction = 'R';

    let mut steps = 0;
    while !deque.len() > 0 {
        if k == directions.len() {
            // println!("Repeating");
            k = 0;
        }

        direction = *directions.get(k).unwrap();


        let src = deque.pop_front().unwrap();

        if src.eq("ZZZ"){
            println!("Found ZZZ");
            break;
        }

        map.contains_key(&src);

        let neis = map.get(&src).unwrap();

        if direction == 'L' {
            deque.push_back(neis.0.to_string());
        } else {
            deque.push_back(neis.1.to_string());
        }

        k += 1;
        steps += 1;
    }


    return steps as i32;
}
