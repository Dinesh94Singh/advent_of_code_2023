use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let lines: Vec<String> = read_to_string("./input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("{}", part1(lines));
}

fn part1(lines: Vec<String>) -> i32 {
    let mut scores: Vec<(u32, u32)> = Vec::new();
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    let char_vals: HashMap<char, u32> = HashMap::from([
        ('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10),
        ('9', 9), ('8', 8), ('7', 7), ('6', 6), ('5', 5),
        ('4', 4), ('3', 3), ('2', 2), ('1', 1),
    ]);

    let mut res: Vec<(u32, u32, u32, &str)> = Vec::new();

    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cards: Vec<char> = parts.get(0).unwrap().chars().collect();
        let bet: u32 = parts.get(1).unwrap().parse().unwrap();

        let mut map: HashMap<char, u32> = HashMap::new();
        let mut sort_rank = 0;

        cards.iter().for_each(|&card| {
            let val = map.entry(card).or_insert(0);
            sort_rank = sort_rank * 10 + char_vals.get(&card).unwrap();
            *val += 1;
        });

        let mut score = 0;

        for (_, &v) in map.iter() {
            if v == 5 {
                score += 25;
            } else if v == 4 {
                score += 16;
            } else if v == 3 {
                score += 9;
            } else if v == 2 {
                score += 4;
            } else if v == 1 {
                score += 1;
            }
        }

        res.push((score, sort_rank, bet, parts.get(0).unwrap()));
    });

    println!("Before Sorting");
    dbg!(&res);

    res.sort_by(|a, b| {
        let s1 = a.0;
        let s2 = b.0;


        if s1 != s2 {
            return s2.partial_cmp(&s1).unwrap();
        }

        let i1 = a.3;
        let i2 = b.3;

        let mut idx = 0;
        let i1_chars: Vec<char> = i1.chars().collect();
        let i2_chars: Vec<char> = i2.chars().collect();

        while idx < i1.len() {
            let ch1 = i1_chars[idx];
            let ch2 = i2_chars[idx];

            if ch1 != ch2 {
                break;
            }

            idx += 1;
        }

        if idx == i1.len() {
            // same cards drawn in same order
            return s2.partial_cmp(&s1).unwrap();
        }

        let k1 = i1_chars[idx];
        let v1 = *char_vals.get(&k1).unwrap();

        let k2 = i2_chars[idx];
        let v2 = *char_vals.get(&k2).unwrap();

        return v2.partial_cmp(&v1).unwrap();
    });


    println!("After Sorting");
    dbg!(&res);

    let mut r = 0;

    let mut total: i32 = res.len() as i32;

    for (_, _, bet, _) in res.iter() {
        r += *bet as i32 * total;
        total -= 1;
    }

    dbg!(r);

    return r;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let t = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let lines: Vec<String> = t.lines().map(String::from).collect();
        part1(lines);
    }
}
