use std::collections::HashSet;

use std::env;

use std::fs::File;
use std::io::Write;


fn part_1(input: &str) -> u32{

    let data = input.split('\n').map(|x| {
        let len = x.len()/2;
        let first: HashSet<char> = x[..len].chars().collect();
        let second: HashSet<char> = x[len..].chars().collect();
        let mut c: u32 = first.intersection(&second).next().cloned().unwrap_or('\0').into();
        if c > 96 {
            c -= 96;
        }
        else if c > 64{
            c -= 38
        }
        c
    });

    let out: u32 = data.sum();
    out
}


fn part_2(input: &str) -> u32{

    let data = &input.split("\n").collect::<Vec<_>>()[..];

    data.chunks(3).map(|group| {
        let c: u32 = group.iter().map(|elf| {
            let h: HashSet<char> = elf.chars().collect();
            h
        }).reduce(|a, b|
            &a & &b
        ).unwrap().drain().map(|x| {
           let i: u32 = x.into();
            if i > 96{
                i - 96
            }
            else if i > 64{
                i - 38
            }
            else {
                0
            }
        }
        ).sum();
        c
    }
    ).sum()

}

fn main(){

    env::set_var("RUST_BACKTRACE", "full");
    let mut output = File::create("src/bin/03/output.txt").unwrap();

    let input = include_str!("input.txt");

    let part_1_res = part_1(input);

    println!("part 1:\n{part_1_res}", part_1_res=part_1_res);
    writeln!(&mut output, "part 1:\n{part_1_res}", part_1_res=part_1_res).unwrap();

    let part_2_res = part_2(input);

    println!("part 2:\n{part_2_res}", part_2_res=part_2_res);
    writeln!(&mut output, "part 2:\n{part_2_res}", part_2_res=part_2_res).unwrap();



}