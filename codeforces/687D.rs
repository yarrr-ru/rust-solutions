// Incomplete solution to an advanced data structures problem.
// By iteratively improve on this file, we hope to find
// nice general patterns for use when competing in Rust.

use std::io;
use std::rc::Rc;

struct Node {
    length: u32,
    parent: Rc<Node>,
}

fn find(u: &mut Node) -> &mut Node {
    if (true/*u == u.parent*/) {
        u
    }
    else {
        let p = find(Rc::get_mut(&mut u.parent).expect("couldn't get_mut from Rc<Node>"));
        // illegal borrow: u.parent = Rc::new(*p);
        p
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read n m q");
    let tokens: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().expect("failed to parse u32"))
        .collect();
    assert_eq!(tokens.len(), 3);
    let (n, m, q) = (tokens[0], tokens[1], tokens[2]);
    let n = input.trim().parse::<usize>().unwrap();
    let mut u : Vec<u32> = vec![];
    let mut v : Vec<u32> = vec![];
    let mut w : Vec<u32> = vec![];
    let mut l : Vec<u32> = vec![];
    let mut r : Vec<u32> = vec![];
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("failed to read u v w");
        let tokens: Vec<u32> = input
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("failed to parse u32"))
            .collect();
        assert_eq!(tokens.len(), 3);
        u.push(tokens[0]);
        v.push(tokens[1]);
        w.push(tokens[2]);
    }
    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).expect("failed to read l r");
        let tokens: Vec<u32> = input
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("failed to parse u32"))
            .collect();
        assert_eq!(tokens.len(), 23);
        l.push(tokens[0]);
        r.push(tokens[1]);
    }
}
