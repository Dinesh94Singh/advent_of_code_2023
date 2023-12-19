use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("./src/input2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!(
        "Total = {}",
        part1(
            lines
                .into_iter()
                .map(|line| line.chars().collect())
                .collect()
        )
    );
}

fn part1(lines: Vec<String>) -> u32 {
    let mut total = 0;
    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let s = parts[0];
        let vals: Vec<u32> = parts[1]
            .split(',')
            .map(|each| each.parse().unwrap())
            .collect();

        let val = calculate_arrangements(&s.to_string(), 0, &vals, 0, "".to_string());
        println!("{} = {}", line, val);
        total += val;
    });
    
    println!("\n\n");
    dbg!(total);
    total
}

fn calculate_arrangements(s: &String, s_idx: usize, vals: &Vec<u32>, v_idx: usize, t: String) -> u32 {
    if s_idx >= s.len() {
        if v_idx == vals.len() {
            println!("t = {}", t);
            return 1;
        }

        return 0;
    }

    if v_idx == vals.len() {
        let cond = s[s_idx..].chars().all(|ch| ch == '.' || ch == '?');
        if cond {
            println!("t = {}", t);
            return 1;
        }

        return 0;
    }

    let ch = s.chars().nth(s_idx).unwrap();

    let mut count = 0;

    if ch == '.' || ch == '?' {
        let x = t.clone() + ".";
        count += calculate_arrangements(s, s_idx + 1, vals, v_idx, x.to_string());
    }

    if ch == '?' || ch == '#' {
        let v: usize = *(vals.get(v_idx).unwrap()) as usize;

        if s_idx + v <= s.len() {
            let substr = &s[s_idx..s_idx + v].to_string();
            let has_no_dot = substr.chars().all(|ch| ch != '.');

            if has_no_dot
                && (s_idx + v == s.len()
                    || (s_idx + v < s.len() && s.chars().nth(s_idx + v).unwrap() != '#'))
            {

                let mut x = t.clone();
                for _ in 0..v {
                    x = x + "#";
                }

                // dbg!(s_idx, v, s.len());
                count += calculate_arrangements(s, s_idx + v + 1, vals, v_idx + 1, x.to_string());
            }
        }
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let t = "???.### 1,1,3";
        let lines: Vec<String> = t.lines().map(String::from).collect();
        println!("{}", part1(lines));
    }
}
