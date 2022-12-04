use std::env;

use std::fs::File;
use std::io::Write;

extern crate regex;
use regex::Regex;


fn part_1(input: &str) -> u32 {
    let pat = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input.split('\n').map(|line| {
        if !pat.is_match(line){
            return 0;
        }
        let caps = pat.captures(line).unwrap();
        let e11: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let e12: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let e21: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let e22: i32 = caps.get(4).unwrap().as_str().parse().unwrap();
        if (e11 <= e21) & (e12 >= e22){
            1
        }
        else if (e11 >= e21) & (e12 <= e22){
            1
        }
        else {
            0
        }
    }).sum()
}

fn part_2(input: &str) -> u32 {
    let pat = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input.split('\n').map(|line| {
        if !pat.is_match(line){
            return 0;
        }
        let caps = pat.captures(line).unwrap();
        let e11: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let e12: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let e21: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let e22: i32 = caps.get(4).unwrap().as_str().parse().unwrap();
        if (e11 >= e21) & (e11 <= e22){
            1
        }
        else if (e21 >= e11) & (e21 <= e12){
            1
        }
        else {
            0
        }
    }).sum()
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let mut output = File::create("src/bin/04/output.txt").unwrap();

    let input = include_str!("input.txt");

    let part_1_res = part_1(input);

    println!("part 1:\n{part_1_res}", part_1_res=part_1_res);
    writeln!(&mut output, "part 1:\n{part_1_res}", part_1_res=part_1_res).unwrap();

    let part_2_res = part_2(input);

    println!("part 2:\n{part_2_res}", part_2_res=part_2_res);
    writeln!(&mut output, "part 2:\n{part_2_res}", part_2_res=part_2_res).unwrap();
}