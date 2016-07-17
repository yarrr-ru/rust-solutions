use std::io;
use std::cmp;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed read_line");
    input.clear();
    io::stdin().read_line(&mut input).expect("failed read_line");
    let mut times: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x|x.parse::<i32>().unwrap())
        .collect();
    times.sort();
    let mut solved: i32 = 0;
    let mut bad_time: i32 = 0;
    let mut cur_time: i32 = 0;
    for time in times {
        if cur_time + time > 710 {
            break
        }
        solved += 1;
        cur_time += time;
        bad_time += cmp::max(0, cur_time - 350);
    }
    println!("{} {}", solved, bad_time);
}
