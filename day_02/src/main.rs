use std::{fs::read_to_string, u32::MIN};
use std::cmp;

fn main() {
    let lines: Vec<String> = read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    part1(lines);
    part2(lines);
}

fn part1(lines: Vec<String>) {
    const ALLOWED_RED_BALLS: u32 = 12;
    const ALLOWED_GREEN_BALLS: u32 = 13;
    const ALLOWED_BLUE_BALLS: u32 = 14;

    let mut total = 0;

    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() > 1 {
            let left_part = parts.get(0);

            let game_info: Vec<&str> = left_part.unwrap().split(' ').collect();
            let game_id: u32 = game_info.get(1).unwrap().parse().unwrap();


            let right_part = parts.get(1).unwrap();
            let games: Vec<&str> = right_part.split(';').collect();

            let mut satisfied = true;

            games.iter().for_each(|game| {
                if !satisfied {
                    return;
                }

                let balls_info: Vec<&str> = game.split(',').collect();
                let mut blue_count = 0;
                let mut red_count = 0;
                let mut green_count = 0;

                balls_info.iter().for_each(|ball_info| {
                    let p: Vec<&str> = ball_info.trim().split(' ').collect();

                    let count: u32 = p.get(0).unwrap().parse().unwrap();
                    let t = p.get(1).unwrap().to_string();
                    let match_t = t.as_str();

                    match match_t {
                        "blue" => blue_count += count,
                        "red" => red_count += count,
                        "green" => green_count += count,
                        _ => println!("Getting Something else"),
                    }
                });

                if blue_count > ALLOWED_BLUE_BALLS || red_count > ALLOWED_RED_BALLS || green_count > ALLOWED_GREEN_BALLS {
                    satisfied = false;
                }
            });

            if satisfied {
                total += game_id;
            }
        }
    });

    println!(" Total = {}", total);
}

fn part2(lines: Vec<String>) {
    const ALLOWED_RED_BALLS: u32 = 12;
    const ALLOWED_GREEN_BALLS: u32 = 13;
    const ALLOWED_BLUE_BALLS: u32 = 14;

    let mut total = 0;

    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() > 1 {
            let left_part = parts.get(0);

            let game_info: Vec<&str> = left_part.unwrap().split(' ').collect();
            let _game_id: u32 = game_info.get(1).unwrap().parse().unwrap();


            let right_part = parts.get(1).unwrap();
            let games: Vec<&str> = right_part.split(';').collect();
            let mut min_blue = MIN;
            let mut min_red = MIN;
            let mut min_green = MIN;

            games.iter().for_each(|game| {
                let balls_info: Vec<&str> = game.split(',').collect();
                balls_info.iter().for_each(|ball_info| {
                    let p: Vec<&str> = ball_info.trim().split(' ').collect();

                    let count: u32 = p.get(0).unwrap().parse().unwrap();
                    let t = p.get(1).unwrap().to_string();
                    let match_t = t.as_str();

                    match match_t {
                        "blue" => {
                            min_blue = cmp::max(min_blue, count);
                        },
                        "red" => {
                            min_red = cmp::max(min_red, count);
                        },
                        "green" => {
                            min_green = cmp::max(min_green, count);
                        },
                        _ => println!("Getting Something else"),
                    }
                });

            });

            let mut prod = 1;
            if min_blue != MIN {
                prod = prod * min_blue;
            }

            if min_green != MIN {
                prod = prod * min_green;
            }

            if min_red != MIN {
                prod = prod * min_red;
            }

            total += prod;
        }
    });

    println!(" Total = {}", total);
}
