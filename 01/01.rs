use std::env;

use std::fs::File;
use std::io::Write;

fn main(){
    env::set_var("RUST_BACKTRACE", "full");

    let mut output = File::create("output.txt").unwrap();

    let data = include_str!("input.txt").split(
        "\r\n\r\n"
    ).map(
        |x| x.lines().map(
            |x| x.parse::<i32>().unwrap()
        )
    );
    let mut sums = data.map(|x| x.sum::<i32>()).collect::<Vec<i32>>();
    sums.sort_by_key(|&x| std::cmp::Reverse(x));
    println!("part 1:\n{max}", max=sums[0]);
    writeln!(&mut output, "part 1:\n{max}", max=sums[0]).unwrap();
    println!("part 2:\n{max3}", max3=sums[0..3].iter().sum::<i32>());
    writeln!(&mut output, "part 2:\n{max3}", max3=sums[0..3].iter().sum::<i32>()).unwrap();


}