#[allow(unused_imports)]
use std::cmp::{min,max};
//const INF: i32 = 0x3f3f3f3f;

struct Scanner {
    buffer: std::collections::VecDeque<String>
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            buffer: std::collections::VecDeque::new()
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            self.buffer = input.split_whitespace()
                .map(ToString::to_string).collect();
        }
        let front = self.buffer.pop_front().unwrap();
        front.parse::<T>().ok().unwrap()
    }
}

const NMAX: usize = 500_001;
const EMAX: usize = NMAX - 1;

struct ARQT {
    d: Vec<Option<i32>>,
    t: Vec<i32>,
    s: Vec<i32>
}

impl ARQT {
    fn new(size: usize) -> ARQT {
        let mut s = vec![1; 2*size];
        for i in (0..size).rev() {
            s[i] = s[i<<1] + s[i<<1|1];
        }
        ARQT {
            d: vec![None; size],
            t: vec![0; 2*size],                       // monoid identity
            s: s
        }
    }
    
    fn apply(&mut self, p: usize, op: i32) {
        self.t[p] = op * self.s[p];                   // hom application
        if p < self.d.len() { self.d[p] = Some(op); } // hom composition
    }
    
    fn push(&mut self, p: usize) {
        for s in (1..32).rev() {
            let i = p >> s;
            if let Some(op) = self.d[i] {
                self.apply(i<<1, op);
                self.apply(i<<1|1, op);
                self.d[i] = None;
            }
        }
    }
    
    fn pull(&mut self, mut p: usize) {
        while p > 1 {
            p >>= 1;
            if self.d[p] == None {
                self.t[p] = self.t[p<<1] + self.t[p<<1|1]; // monoid op
            }
        }
    }
    
    fn modify(&mut self, mut l: usize, mut r: usize, op: i32) {
        l += self.d.len(); r += self.d.len();
        let (l0, r0) = (l, r);
        self.push(l0); self.push(r0);
        while l <= r {
          if l & 1 == 1 { self.apply(l, op); l += 1; }
          if r & 1 == 0 { self.apply(r, op); r -= 1; }
          l >>= 1; r >>= 1;
        }
        self.pull(l0); self.pull(r0);
    }
    
    fn query(&mut self, mut l: usize, mut r: usize) -> i32 {
        l += self.d.len(); r += self.d.len();
        self.push(l); self.push(r);
        let mut res = 0;                                     // monoid identity
        while l <= r {
            if l & 1 == 1 { res = res + self.t[l]; l += 1; } // monoid op
            if r & 1 == 0 { res = self.t[r] + res; r -= 1; } // monoid op
            l >>= 1; r >>= 1;
        }
        res
    }
}

struct Graph {
    first: [Option<usize>; NMAX],
    next: [Option<usize>; 2*EMAX],
    endp: [usize; 2*EMAX],
    l: [usize; NMAX],
    r: [usize; NMAX],
    p: [usize; NMAX]
}

impl Graph {
    fn new() -> Graph {
        Graph {
            first: [None; NMAX],
            next: [None; 2*EMAX],
            endp: [0; 2*EMAX],
            l: [0; NMAX],
            r: [0; NMAX],
            p: [0; NMAX]
        }
    }
    
    fn dfs(&mut self, u: usize, time: &mut usize) {
        *time += 1;
        self.l[u] = *time;
        
        let mut e = self.first[u];
        while let Some(edge_id) = e {
            let v = self.endp[edge_id ^ 1];
            if self.l[v] == 0 {
                self.p[v] = self.l[u];
                self.dfs(v, time);
            }
            e = self.next[edge_id];
        }
        
        self.r[u] = *time;
    }
}

fn main1() {
    let mut scan = Scanner::new();
    let mut tree = Graph::new();
    let n = scan.next::<usize>();
    for e in 0..2*(n-1) {
        let u = scan.next::<usize>() - 1;
        tree.endp[e] = u;
        tree.next[e] = tree.first[u];
        tree.first[u] = Some(e);
    }
    tree.dfs(0, &mut 0);
    
    let mut arqt = ARQT::new(NMAX);
    let q = scan.next::<usize>();
    for _ in 0..q {
        let c = scan.next::<usize>();
        let v = scan.next::<usize>() - 1;
        let (p, l, r) = (tree.p[v], tree.l[v], tree.r[v]);
        let len = (1 + r - l) as i32;
        let full = arqt.query(l, r) == len;
        if c == 1 {
            if !full {
                arqt.modify(p, p, 0);
                arqt.modify(l, r, 1);
            }
        }
        else if c == 2 {
            arqt.modify(l, l, 0);
        }
        else {
            println!("{}", if full { 1 } else { 0 } );
        }
    }
}

fn main() {
    std::thread::Builder::new().stack_size(1 << 26)
        .spawn(main1).unwrap().join().unwrap();
}
