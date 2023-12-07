use std::{fs::read_to_string, u64};

fn main() {
    let lines: Vec<String> = read_to_string("./input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("{}", part1(lines));

    let lines: Vec<String> = read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("{}", part2(lines));
}


fn part1(lines: Vec<String>) -> i32 {
    let time_info = lines.get(0).unwrap();
    let distance_info = lines.get(1).unwrap();

    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();

    let t_info: Vec<&str> = time_info.split(":").collect();

    t_info.get(1).unwrap().split_whitespace().into_iter().for_each(|time| {
        times.push(time.parse().unwrap());
    });


    let d_info: Vec<&str> = distance_info.split(":").collect();

    d_info.get(1).unwrap().split_whitespace().into_iter().for_each(|dist| {
        distances.push(dist.parse().unwrap());
    });

    let mut res = 1;
    times.iter().zip(distances.iter()).for_each(|(&time, &dist)| {
        let mut ways = 0;
        for held in 0..time {
            let speed = held; // boat travels at the speed of held units
            let distance_covered = speed * (time - held);

            if distance_covered > dist {
                ways += 1;
            }
        }

        res = res * ways;
    });

    return res;
}

fn part2(lines: Vec<String>) -> i32 {

    let time_info = lines.get(0).unwrap();
    let distance_info = lines.get(1).unwrap();

    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();

    let t_info: Vec<&str> = time_info.split(":").collect();
    let mut time: String = "".to_owned();

    t_info.get(1).unwrap().split_whitespace().into_iter().for_each(|t| {
        time += t;
    });

    let mut distance: String = "".to_owned();

    let d_info: Vec<&str> = distance_info.split(":").collect();

    d_info.get(1).unwrap().split_whitespace().into_iter().for_each(|d| {
        distance += d;
    });

    println!("Distance = {}", distance);
    println!("Time = {}", time);

    let t = time.parse().unwrap();
    let d = distance.parse::<u64>().unwrap();

    let mut ways = 0;
    for held in 0..t {
        let speed = held; // boat travels at the speed of held units
        let distance_covered = speed * (t - held);

        if distance_covered > d {
            ways += 1;
        }
    }

    return ways;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let ip = "Time:  7  15   30
Distance:  9  40  200";

        println!("{}", part1(ip.lines().map(String::from).collect()));
    }
}
