use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let lines: Vec<String> = read_to_string("./input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    part2(lines);
}

fn part1(lines: Vec<String>) {
    let mut numbers: Vec<u32> = Vec::new();

    for each_line in lines {
        let chars: Vec<char> = each_line.chars().collect();

        let mut first_digit = 0;
        let mut last_digit = 0;

        let mut digit_count = 0;

        for each_char in chars {
            if each_char.is_digit(10) {
                if digit_count == 0 {
                    first_digit = each_char.to_digit(10).unwrap();
                } else {
                    last_digit = each_char.to_digit(10).unwrap();
                }

                digit_count += 1;
            }
        }

        if digit_count < 2 && digit_count > 0 {
            last_digit = first_digit;
        } else if digit_count == 0 {
            continue;
        }

        let number = first_digit * 10 + last_digit;
        numbers.push(number);
    }

    let res: u32 = solve(numbers);
    println!("Part1 Result = {}", res);
}

fn part2(lines: Vec<String>) {
    let mut numbers: Vec<u32> = Vec::new();
    let mut number_in_words: HashMap<String, i32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    for each_line in lines {
        let mut i: usize = 0;
        let mut first_number = 0;
        let mut last_number = 0;

        let mut found = false;

        while i < each_line.len() {
            let start = cmp::min(i + 3, each_line.len());
            let end = cmp::min(start + 5, each_line.len());

            let ch = each_line.get(
                i..i+1
            ).unwrap();

            found = false;

            match ch.to_string().parse() {
                Result::Ok(v) => {
                    found = true;
                    first_number = v;
                },
                _ => {

                }
            }

            if found {
                break;
            }

            for j in start..end {
                let sub_string: &str = &each_line[i..j];

                if number_in_words.contains_key(sub_string) {
                    found = true;
                    let val = *number_in_words.get(sub_string).unwrap();
                    first_number = val;
                    break;
                }
            }

            if found {
                break;
            }

            i += 1;
        }

        let length = each_line.len();

        i = each_line.len() - 1;

        found = false;

        while i >= 0 {

            let ch = each_line.get(
                i..i+1
            ).unwrap();

            found = false;

            match ch.to_string().parse() {
                Result::Ok(v) => {
                    found = true;
                    last_number = v;
                },
                _ => {

                }
            }

            if found {
                break;
            }

            let start = cmp::min(length, i + 2);
            let end = cmp::min(length, i + 5);

            for j in start..end {
                let sub_string: &str = &each_line[i..j + 1];

                if number_in_words.contains_key(sub_string) {
                    found = true;
                    last_number = *number_in_words.get(sub_string).unwrap();
                    break;
                }
            }

            if found {
                break;
            }

            if i == 0 {
                break;
            }

            i -= 1;
        }

        let number = first_number * 10 + last_number;
        numbers.push(number as u32);
    }

    println!("Part2 - {}", solve(numbers));
}

fn solve(data: Vec<u32>) -> u32 {
    let mut total: u32 = 0;

    for each_number in data {
        total += each_number;
    }

    return total;
}
