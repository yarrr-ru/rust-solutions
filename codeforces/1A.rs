use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed read_line");
    let tokens: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x|x.parse::<i64>().unwrap())
        .collect();
    let (n, m, a) = (tokens[0], tokens[1], tokens[2]);
    let result = ((n + a - 1) / a) * ((m + a - 1) / a);
    println!("{}", result);
}
