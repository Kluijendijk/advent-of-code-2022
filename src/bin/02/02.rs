use std::env;

use std::fs::File;
use std::io::Write;

extern crate regex;
use regex::{Captures, Regex};

fn main(){
    env::set_var("RUST_BACKTRACE", "full");
    let mut output = File::create("src/bin/02/output.txt").unwrap();

    // 0 for loss, 3 for draw, 6 for win
    // 1 for rock, 2 for paper, 3 for scissors
    // A, X -> Rock, C,Y -> Scissors, B,Z -> Paper
    // points for round: play_points[you] + win_points[(opp - you + 1) % 3]

    let data = include_str!("input.txt");

    let pattern = Regex::new("[A-Z]").unwrap();
    let points = pattern.replace_all(data, |cap: &Captures|{
       match &cap[0] {
           "A" => "0",
           "B" => "1",
           "C" => "2",
           "X" => "0",
           "Y" => "1",
           "Z" => "2",
           _ => "",
       }
    });
    let points_pt_1 = points.split("\n").map(|x| {
        let x_split = x.trim().split(" ").collect::<Vec<&str>>();
        if x_split.len() > 1 {
            let x_opp: i32 = x_split[0].parse().unwrap_or(-1);
            let x_you: i32 = x_split[1].parse().unwrap_or(-1);
            if x_opp * x_you < 0 {
                0
            }
            else {
                let out = 3*((x_you-x_opp+4)%3) + (x_you+1) ;
                // println!("o:{o} y:{y} ot:{ot} yt:{yt}", o=x_opp, y=x_you, ot=3*((x_you-x_opp+4)%3), yt=(x_you+1));
                out
            }
        }
        else {
            0
        }
    }).collect::<Vec<i32>>();
    let total_score_pt_1: i32 = points_pt_1.iter().sum();

    let points_pt_2 = points.split("\n").map(|x| {
        let x_split = x.trim().split(" ").collect::<Vec<&str>>();
        if x_split.len() > 1 {
            let x_opp: i32 = x_split[0].parse().unwrap_or(-1);
            let x_you: i32 = x_split[1].parse().unwrap_or(-1);
            if x_opp * x_you < 0 {
                0
            }
            else {
                let out = 3*x_you + 1+((x_opp + x_you + 5)%3);
                 // println!("o:{o} y:{y} ot:{ot} yt:{yt}", o=x_opp, y=x_you, ot=1+((x_opp + x_you + 5)%3), yt=3*x_you);
                out
            }
        }
        else {
            0
        }
    }).collect::<Vec<i32>>();

    let total_score_pt_2: i32 = points_pt_2.iter().sum();


    println!("part 1:\n{score}", score=total_score_pt_1);
    if let Err(e) = writeln!(&mut output, "part 1:\n{score}", score=total_score_pt_1) {
        println!("Writing error: {e}", e=e.to_string())
    }
    println!("part 2:\n{score}", score=total_score_pt_2);
    if let Err(e) = writeln!(&mut output, "part 2:\n{score}", score=total_score_pt_2) {
        println!("Writing error: {e}", e = e.to_string())
    }
    ()
}