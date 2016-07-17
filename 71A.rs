use std::io;

fn i18n(word: &str) -> String {
    let mut result: String;

    if word.len() > 10 {
        result = String::new();
        result.push(word.chars().next().unwrap());
        result.push_str(&format!("{}", word.len() - 2));
        result.push(word.chars().rev().next().unwrap());
    } else {
        result = String::from(word);
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed read_line");
    let n = input.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("failed read_line");
        let word = input.trim();
        println!("{}", i18n(word));
    }
}
