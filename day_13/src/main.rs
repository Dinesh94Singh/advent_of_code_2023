use std::fs::read_to_string;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::Write;
use std::slice::Windows;
use nom::IResult;
use nom::bytes::complete::{take_until, take_till};
use itertools::Itertools; // for tuple window iteration

// Get Paragraphs
fn parse_blocks(input: &str) -> IResult<&str, Vec<&str>> {
    let mut result = Vec::new();
    let mut remaining = input;
    dbg!(&input);
    loop {
        if remaining.is_empty() {
            break;
        }
        let (rest, chunk) = take_until("\n\n")(remaining)?;
        dbg!(&rest, &chunk);

        let (left_over, _) = take_till(|x| x == '#' || x == '.')(rest)?;
        dbg!(&left_over);

        remaining = left_over;
        dbg!(&remaining);
        if !chunk.is_empty() {
            result.push(chunk);
        } else {
            break;
        }
    }
    IResult::Ok((remaining, result))
}

fn main() {
    let input_str = read_to_string("./src/input2.txt").unwrap();
    // let _blocks: IResult<&str, Vec<&str>> = parse_blocks(&input_str); // How to use Nom

    let res = input_str.split("\n\n")
        .flat_map(detect_fold).fold(
            0usize,
            |mut acc, item| match item.0.as_str() {
                "Horizontal" => {
                    acc += 100 * item.1;
                    acc
                }
                "Vertical" => {
                    acc += item.1;
                    acc
                }
                _ => {
                    println!("Here");
                    acc
                }
            }
        );

    println!("Result = {}", res);

    let res_part_2 = input_str.split("\n\n")
        .flat_map(detect_fold_part_2).fold(
            0usize,
            |mut acc, item| match item.0.as_str() {
                "Horizontal" => {
                    acc += 100 * item.1;
                    acc
                }
                "Vertical" => {
                    acc += item.1;
                    acc
                }
                _ => {
                    println!("Here");
                    acc
                }
            }
        );

    println!("Result = {}", res_part_2);
}

fn horizontal_fold_part_2(input: &str) -> Option<(String, usize)> {
    let lines: Vec<&str> = input.lines().collect();
    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|((index_1, line_1), (index_2, line_2))| { // find the lines where 2 adj lines are
                                                           // same. They would be same, if they are
                                                           // mirrors
            line_1 == line_2
                || line_1.chars().zip(line_2.chars()).filter(|(a,b)| a != b).count() <= 1
        })
        .find_map(|((index_1, line_1), (index_2, line_2))| { // Find map is same as filter_map.next()
            let bottom_half = (&lines[0..=index_1]).iter().map(|line| line.chars()).rev();
            let top_half = (&lines[index_2..]).iter().map(|line| line.chars());

            (bottom_half
                .flatten()
                .zip(top_half.flatten())
                .filter(|(a, b)| a != b)
                .count() == 1)
                .then_some(("Horizontal".to_string(), index_1 + 1))
        });

    return result;
}

fn vertical_fold_part_2(input: &str) -> Option<(String, usize)> {
    let mut columns_iter = input.lines().map(|line| line.chars()).collect::<Vec<_>>();

    // Convert into Vec<Vec<char>> from Vec<Char'>
    let columns = std::iter::from_fn(move || {
        let mut items = vec![];

        for chars_iter in &mut columns_iter {
            match chars_iter.next() { // it will always have some rows
                Some(item) => {
                    items.push(item);
                }
                None => return None, // if no chars, skip adding
            }
        }
        Some(items)
    }).collect::<Vec<Vec<char>>>();

    let result = columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((index_a, line_a), (index_b, line_b))| { // Will get 2 Vec<Char> representing 2
                                                           // columns. if the 2 Vec<Char> are same,
                                                           // compare the other parts
            line_a == line_b
                || line_a
                .iter()
                .zip(line_b.iter())
                .filter(|(a, b)| a != b).count() <= 1
        })
        .find_map(|((index_a, line_a), (index_b, line_b))| {
            let left_half = (&columns[0..=index_a]).iter().rev(); // Iter of Vec<Vec<Char>> from
                                                                  // idx 0..=index_1 (where the
                                                                  // previous match found
            let right_half = (&columns[index_b..]).iter();

            (left_half.flatten()
                .zip(right_half.flatten())
                .filter(|(a,b)| a != b)
                .count() == 1) // at most 1 diff only
                .then_some(("Vertical".to_string(), index_a + 1))
        });

    return result;
}

fn horizontal_fold(input: &str) -> Option<(String, usize)> {
    let lines: Vec<&str> = input.lines().collect();
    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|((index_1, line_1), (index_2, line_2))| { // find the lines where 2 adj lines are
                                                           // same. They would be same, if they are
                                                           // mirrors
            line_1 == line_2
        })
        .find_map(|((index_1, line_1), (index_2, line_2))| { // Find map is same as filter_map.next()
            let bottom_half = (&lines[0..=index_1]).iter().rev();
            let top_half = (&lines[index_2..]).iter();

            bottom_half
                .zip(top_half)
                .all(|(f_v, s_v)| f_v == s_v) // if all columns of the both halfs are equal
                .then_some(("Horizontal".to_string(), index_1 + 1))
        });

    return result;
}

fn vertical_fold(input: &str) -> Option<(String, usize)> {
    let mut columns_iter = input.lines().map(|line| line.chars()).collect::<Vec<_>>();

    // Convert into Vec<Vec<char>> from Vec<Char'>
    let columns = std::iter::from_fn(move || {
        let mut items = vec![];

        for chars_iter in &mut columns_iter {
            match chars_iter.next() { // it will always have some rows
                Some(item) => {
                    items.push(item);
                }
                None => return None, // if no chars, skip adding
            }
        }
        Some(items)
    }).collect::<Vec<Vec<char>>>();

    let result = columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((index_a, line_a), (index_b, line_b))| { // Will get 2 Vec<Char> representing 2
                                                           // columns. if the 2 Vec<Char> are same,
                                                           // compare the other parts
            line_a == line_b
        })
        .find_map(|((index_a, line_a), (index_b, line_b))| {
            let left_half = (&columns[0..=index_a]).iter().rev(); // Iter of Vec<Vec<Char>> from
                                                                  // idx 0..=index_1 (where the
                                                                  // previous match found
            let right_half = (&columns[index_b..]).iter();

            left_half
                .zip(right_half)
                .all(|(a,b)| a == b)
                .then_some(("Vertical".to_string(), index_a + 1))
        });

    return result;
}

fn detect_fold(input: &str) -> Option<(String, usize)> {
    horizontal_fold(input).or(vertical_fold(input))
}

fn detect_fold_part_2(input: &str) -> Option<(String, usize)> {
    horizontal_fold_part_2(input).or(vertical_fold_part_2(input))
}
