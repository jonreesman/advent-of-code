#![allow(dead_code)]
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Mapping {
    dest_start: i64,
    src_start: i64,
    range: i64
}

impl Mapping {
    fn new(dest_start: i64, src_start: i64, range: i64) -> Self {
        Mapping {
            dest_start,
            src_start,
            range
        }
    }

    fn dest_range(&self) -> (i64, i64) {
        return (self.dest_start, self.dest_start + self.range - 1)
    }

    fn src_range(&self) -> (i64, i64) {
        return (self.src_start, self.src_start + self.range - 1)
    }

    fn difference(&self) -> i64 {
        return self.dest_start - self.src_start
    }
}

#[allow(dead_code)]
struct Mapper {
    name: String,
    mappings: Vec<Mapping>,
}

impl Mapper {
    fn new(name: String) -> Self {
        Mapper {
            name,
            mappings: Vec::new(),
        }
    }

    fn get_dest(&self, num: i64) -> i64 {

        let mut val: i64 = num;
        for mapping in self.mappings.iter() {
            if mapping.src_start < num && num < mapping.src_start + mapping.range {
                val = mapping.dest_start + num - mapping.src_start
            }
        }

        val
    }

    fn add_mapping(&mut self, mapping: Mapping) {
        self.mappings.push(mapping)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_file() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    if let Ok(line_buff) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in line_buff {
            if let Ok(ip) = line {
                lines.push(ip);
            }
        }
    }

    lines
}

fn get_mappers() -> Vec<Mapper> {
    let mut lines = get_file();
    lines.remove(0);
    lines.remove(0);

    let mut mappers: Vec<Mapper> = Vec::new();
    let mut curr_mapper = None;

    for line in lines.iter() {
        if line.contains("map") {
            let split_line = line.split_whitespace().map(String::from).collect::<Vec<String>>();
            let name = split_line.first().unwrap();
            curr_mapper = Some(Mapper::new(name.to_string()));
        } else if line.eq_ignore_ascii_case("\n") || line.is_empty() {
            mappers.push(curr_mapper.unwrap());
            curr_mapper = None;
        } else {
            if let Some(ref mut map ) = curr_mapper {
                let nums: Vec<i64> = line.replace("\n", "").split_whitespace().map(String::from).map(|x| x.parse::<i64>().unwrap()).collect();
                let mapping = Mapping::new(nums[0], nums[1], nums[2]);
                map.add_mapping(mapping);
            }
        }
    }
    if let Some(map ) = curr_mapper {
        mappers.push(map);
    }

    mappers
}

fn get_seeds() -> Vec<i64> {
    let mut lines: Vec<String> = get_file();

    let seeds_string: String = lines.remove(0).replace("\n", "");
    let seeds = seeds_string.split(":")
        .map(String::from)
        .collect::<Vec<String>>()
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(String::from)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    seeds
}

fn part_one() {
    let seeds = get_seeds();
    let mut mappers = get_mappers();


    let mut lowest_location = i64::MAX;
    for seed in seeds {
        println!("{}", seed);
        let mut num = seed;
        for mapper in mappers.iter_mut() {
            num = mapper.get_dest(num)
        }
        if num < lowest_location {
            lowest_location = num
        }
    }

    println!("Lowest Location: {}", lowest_location)

}


fn part_two() {
    let seeds: Vec<i64> = get_seeds();
    let mappers = get_mappers();

    let mut seed_ranges = VecDeque::new();

    for nums in seeds.chunks(2) {
        seed_ranges.push_back((nums[0], nums[0] + nums[1] - 1));
    }

    let mut new_ranges = VecDeque::new();
    for mapper in mappers {
        new_ranges.clear();
        while seed_ranges.len() > 0 {
            let (start, end) = seed_ranges.pop_front().unwrap();
            for mapping in mapper.mappings.iter() {
                let (m_start, m_end) = mapping.src_range();

                if end < m_start || start > m_end {
                    // No overlap, skip range
                    continue
                }

                let possible_start = i64::max(start, m_start);
                let possible_end = i64::min(end, m_end);

                if possible_end - possible_start > 0 {
                    new_ranges.push_back((possible_start + mapping.difference(), possible_end + mapping.difference() - 1));
                }
                if possible_start - start > 0 {
                    new_ranges.push_back((start, possible_start - 1));
                }
                if end - possible_end > 0 {
                    new_ranges.push_back((possible_end, end));
                }
            }
        }
        seed_ranges.clone_from(&new_ranges);
    }

    let mut vec_ranges: Vec<(i64, i64)> = new_ranges.into();

    vec_ranges.sort_by_key(|k| k.to_owned().0);

    println!("{}", vec_ranges.first().unwrap().0)

}

fn main() {
    part_two()
}
