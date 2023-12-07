use std::fs::read_to_string;
use std::cmp;

fn main() {
    let lines: String = read_to_string("./input.txt")
        .unwrap();

    println!("{}", part1(lines));
}


fn part1(lines: String) -> u64 {
    let partitions: Vec<&str> = lines.split("\n\n").collect();

    let seeds_info = partitions.get(0).unwrap();
    let seed_to_soil_info = partitions.get(1).unwrap();
    let soil_to_fertilizer_info = partitions.get(2).unwrap();
    let fertilizer_to_water_info = partitions.get(3).unwrap();
    let water_to_light_info = partitions.get(4).unwrap();
    let light_to_temp_info = partitions.get(5).unwrap();
    let temp_to_humidity_info = partitions.get(6).unwrap();
    let humidity_to_location_info = partitions.get(7).unwrap();


    let si: Vec<&str> = seeds_info.split(":").collect();
    let seeds: Vec<u64> = si.get(1).unwrap().split_whitespace().map(|e| {
        e.parse().unwrap()
    }).collect();

    let seeds_to_soil: Vec<(u64, u64, u64)> = translate(seed_to_soil_info.to_string());
    let soil_to_fertilizer: Vec<(u64, u64, u64)> = translate(soil_to_fertilizer_info.to_string());
    let fertilizer_to_water: Vec<(u64, u64, u64)> = translate(fertilizer_to_water_info.to_string());
    let water_to_light: Vec<(u64, u64, u64)> = translate(water_to_light_info.to_string());
    let light_to_temp: Vec<(u64, u64, u64)> = translate(light_to_temp_info.to_string());
    let temp_to_humidity: Vec<(u64, u64, u64)> = translate(temp_to_humidity_info.to_string());
    let humidity_to_location: Vec<(u64, u64, u64)> = translate(humidity_to_location_info.to_string());

    println!("Done Translating");

    let mut res = u64::MAX;



    let first_round_data = get_next_round_data(seeds, seeds_to_soil);

    let second_round_data = get_next_round_data(first_round_data, soil_to_fertilizer);

    let third_round_data = get_next_round_data(second_round_data, fertilizer_to_water);

    let fourth_round_data = get_next_round_data(third_round_data, water_to_light);

    let fifth_round_data = get_next_round_data(fourth_round_data, light_to_temp);

    let sixth_round_data = get_next_round_data(fifth_round_data, temp_to_humidity);

    let seventh_round_data = get_next_round_data(sixth_round_data, humidity_to_location);

    seventh_round_data.iter().for_each(|&data| {
        res = cmp::min(res, data);
    });

    return res;
}



fn get_next_round_data(prev_round_data: Vec<u64>, data: Vec<(u64, u64, u64)>) -> Vec<u64> {
    let mut next_round_data: Vec<u64> = Vec::new();

    prev_round_data.iter().for_each(|&each| {
        let mut found = false;

        data.iter().for_each(|&e| {
            let dest = e.0;
            let source = e.1;
            let range = e.2;

            if source <= each && each < source + range {
                next_round_data.push(each - source + dest);
                found = true;
            }
        });

        if !found { // no range can be found, so default to same value
            next_round_data.push(each);
        }
    });

    return next_round_data;
}

fn translate(lines: String) -> Vec<(u64, u64, u64)> {
    let mut data: Vec<(u64, u64, u64)> = Vec::new();

    let parts: Vec<&str> = lines.split(":").collect();
    let part = parts.get(1).unwrap().trim();

    part.lines().for_each(|line| {
        let info: Vec<&str> = line.split(" ").collect();

        let dest_start: u64 = info.get(0).unwrap().parse().unwrap();
        let source_start: u64 = info.get(1).unwrap().parse().unwrap();
        let range : u64 = info.get(2).unwrap().parse().unwrap();

        data.push((dest_start, source_start, range));
    });

    return data;
}

