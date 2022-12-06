use std::env;

use std::fs::File;
use std::io::Write;

extern crate regex;
use regex::Regex;

fn part_1(input: &str) -> String {
    let data: Vec<Vec<&str>> = input.split("\r\n\r\n").map(|x| x.split('\n').collect()).collect();

    let whitespace = Regex::new(r" +").unwrap();

    let stacks = whitespace.replace_all(data[0].last().unwrap(), ",");
    let stacks = stacks.split(",");

    let num_stacks: usize = stacks.last().unwrap().parse().unwrap();

    let mut stacks:Vec<String> = Vec::new();

    for s in 0..num_stacks{
        stacks.push("".to_string());
    }

    // initialize
    for line in data[0].split_last().unwrap().1.iter(){
        for i in 0..num_stacks{
            if line.len() > 4*i{
                let substr = &line[4*i..4*(i+1)-1];
                if !whitespace.is_match(substr){
                    stacks[i].push_str(&substr.chars().nth(1).unwrap().to_string());
                }
            }
        }
    }

    for i in 0..num_stacks{
        stacks[i] = stacks[i].chars().rev().collect::<String>();
    }

    // moves
    let move_patt = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in &data[1]{
        if move_patt.is_match(line){
            let caps = move_patt.captures(line).unwrap();
            let sfrom: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let sfrom = sfrom - 1;
            let sto: usize = caps.get(3).unwrap().as_str().parse().unwrap();
            let sto = sto - 1;
            for _ in 0..caps.get(1).unwrap().as_str().parse().unwrap(){
                let borrowed_letter = stacks[sfrom].pop().unwrap().to_string();
                stacks[sto].push_str(&borrowed_letter);
            }
            // for s in &stacks{
            //     println!("{}", s);
            // }

        }
    }
    let mut out = "".to_string();
    for mut s in stacks{
        out.push_str(&s.pop().unwrap().to_string());
    }
    out

}


fn part_2(input: &str) -> String {
    let data: Vec<Vec<&str>> = input.split("\r\n\r\n").map(|x| x.split('\n').collect()).collect();

    let whitespace = Regex::new(r" +").unwrap();

    let stacks = whitespace.replace_all(data[0].last().unwrap(), ",");
    let stacks = stacks.split(",");

    let num_stacks: usize = stacks.last().unwrap().parse().unwrap();

    let mut stacks:Vec<String> = Vec::new();

    for s in 0..num_stacks{
        stacks.push("".to_string());
    }

    // initialize
    for line in data[0].split_last().unwrap().1.iter(){
        for i in 0..num_stacks{
            if line.len() > 4*i{
                let substr = &line[4*i..4*(i+1)-1];
                if !whitespace.is_match(substr){
                    stacks[i].push_str(&substr.chars().nth(1).unwrap().to_string());
                }
            }
        }
    }

    for i in 0..num_stacks{
        stacks[i] = stacks[i].chars().rev().collect::<String>();
    }

    // moves
    let move_patt = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in &data[1]{
        if move_patt.is_match(line){
            let caps = move_patt.captures(line).unwrap();
            let sfrom: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let sfrom = sfrom - 1;
            let sto: usize = caps.get(3).unwrap().as_str().parse().unwrap();
            let sto = sto - 1;
            let mut add_str = "".to_string();
            for _ in 0..caps.get(1).unwrap().as_str().parse().unwrap(){
                let borrowed_letter = stacks[sfrom].pop().unwrap().to_string();
                add_str.push_str(&borrowed_letter);
            }
            stacks[sto].push_str(&add_str.chars().rev().collect::<String>());
            // for s in &stacks{
            //     println!("{}", s);
            // }

        }
    }
    let mut out = "".to_string();
    for mut s in stacks{
        out.push_str(&s.pop().unwrap().to_string());
    }
    out
}


fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let mut output = File::create("src/bin/05/output.txt").unwrap();

    let input = include_str!("input.txt");

    let part_1_res = part_1(input);

    println!("part 1:\n{part_1_res}", part_1_res=part_1_res);
    writeln!(&mut output, "part 1:\n{part_1_res}", part_1_res=part_1_res).unwrap();

    let part_2_res = part_2(input);

    println!("part 2:\n{part_2_res}", part_2_res=part_2_res);
    writeln!(&mut output, "part 2:\n{part_2_res}", part_2_res=part_2_res).unwrap();
}