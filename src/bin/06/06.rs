use std::env;

use std::fs::File;
use std::io::Write;

use std::collections::HashSet;
use std::hash::Hash;


fn part_1(input: &str) -> i32 {
    let data = input[3..].chars();

    let mut prev: String = " ".to_owned() + &input[0..3].to_string();
    let mut ind = 3;

    for letter in data{
        ind += 1;
        if letter == '\n'{
            continue
        }
        prev.remove(0);
        prev = prev + &String::from(letter);
        if prev[1..].contains(prev.chars().nth(0).unwrap()) |
            prev[..3].contains(letter) |
            (prev.chars().nth(1).unwrap() == prev.chars().nth(2).unwrap()){
            continue
        }
        else{
            break
        }

    }

    ind
}


fn part_2(input: &str) -> i32 {
    let data = input.chars();

    let mut prev: String = "11111111111111".to_owned();
    let mut ind = 0;

    for letter in data{
        ind += 1;
        if letter == '\n'{
            continue
        }
        prev.remove(0);
        prev = prev + &String::from(letter);
        let h: HashSet<char> = prev.chars().collect();
        if h.len() >= 14{
            break
        }
    }

    ind
}


fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let mut output = File::create("src/bin/06/output.txt").unwrap();

    let input = include_str!("input.txt");

    let part_1_res = part_1(input);

    println!("part 1:\n{part_1_res}", part_1_res=part_1_res);
    writeln!(&mut output, "part 1:\n{part_1_res}", part_1_res=part_1_res).unwrap();

    let part_2_res = part_2(input);

    println!("part 2:\n{part_2_res}", part_2_res=part_2_res);
    writeln!(&mut output, "part 2:\n{part_2_res}", part_2_res=part_2_res).unwrap();
}