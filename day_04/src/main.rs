use std::{fs::read_to_string, collections::{HashSet, HashMap}};

fn main() {
    let lines: Vec<String> = read_to_string("./input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    println!("{}", part2(lines));
}


fn part2(lines: Vec<String>) -> i32 {
    let mut total = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut original_cards_count = 0;

    lines.iter().for_each(|line| {
        original_cards_count += 1;

        let game: Vec<&str> = line.split(":").collect();

        let info = game.get(1).unwrap();

        let game_info: Vec<&str> = game.get(0).unwrap()
            .split_whitespace()
            .collect();

        let game_id: i32 = game_info
            .get(1)
            .unwrap()
            .parse()
            .unwrap();

        let parts: Vec<&str> = info.split("|").collect();

        let valid_cards: HashSet<i32> = parts.get(0).unwrap().trim()
            .split_whitespace()
            .map(|digit| digit.parse::<i32>().unwrap())
            .collect();

        let mut count: u32 = 0;
        parts.get(1).unwrap().trim()
            .split_whitespace()
            .for_each(|d| {
                let digit = d.parse::<i32>().unwrap();

                if valid_cards.contains(&digit) {
                    // Score - 1, 2, 4, 8, 16
                    count = count + 1;
                }
            });

        if count > 0 {
            for i in 1..count + 1 {
                let key = game_id + i as i32;
                if !map.contains_key(&key) {
                    map.insert(key, 0);
                }

                let mut copies = 0;

                if map.contains_key(&game_id) {
                    copies += *map.get(&game_id).unwrap();
                }

                // println!("for game id = {}, copy_id = {} , {:?}", game_id, key, copies);


                map.insert(key, *map.get(&key).unwrap() + copies + 1);
            }

            let base: i32 = 2;
            total += base.pow(count - 1);
        }
    });

    let mut copy_count = 0;

    for (_, v) in map.iter() {
        copy_count += v;
    }
    return original_cards_count + copy_count;
}

fn part1(lines: Vec<String>) -> i32 {
    let mut total = 0;
    lines.iter().for_each(|line| {
        let game: Vec<&str> = line.split(":").collect();

        let info = game.get(1).unwrap();

        let parts: Vec<&str> = info.split("|").collect();

        /*

          Converting String to Vec<i32>
          let valid_cards: Vec<i32> = parts.get(0).unwrap().trim()
            .split_whitespace()
            .map(|digit| digit.parse::<i32>().unwrap())
            .collect();

        */

        let valid_cards: HashSet<i32> = parts.get(0).unwrap().trim()
            .split_whitespace()
            .map(|digit| digit.parse::<i32>().unwrap())
            .collect();

        let mut count: u32 = 0;
        parts.get(1).unwrap().trim()
            .split_whitespace()
            .for_each(|d| {
                let digit = d.parse::<i32>().unwrap();

                if valid_cards.contains(&digit) {
                    // Score - 1, 2, 4, 8, 16
                    count = count + 1;
                }
            });

        if count > 0 {
            let base: i32 = 2;
            total += base.pow(count - 1);
        }
    });
    return total;
}
