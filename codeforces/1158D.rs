// A concise solution to Codeforces 1158D, which is very similar to an old ICPC regionals problem:
// https://icpcarchive.ecs.baylor.edu/index.php?option=com_onlinejudge&Itemid=8&category=759&page=show_problem&problem=5721
const INF: i64 = 0x3f3f3f3f;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn cross((x0, y0): (i64, i64),
         (x1, y1): (i64, i64),
         (x2, y2): (i64, i64)) -> i64 {
    (x1-x0)*(y2-y0) - (x2-x0)*(y1-y0)
}

fn maint() {
    let mut scan = Scanner::default();
    let n = scan.next();
    let p: Vec<_> = (0..n).map(|_| (scan.next(), scan.next()))
                          .collect();
    let s = format!("R{}R", scan.next::<String>());
    
    let mut taken = vec![false; n];
    let mut last = (INF, INF);
    
    for turn in s.chars() {
        let sgn = if turn == 'L' { 1 } else { -1 };
        let i = (0..n).filter(|&i| !taken[i])
                      .max_by(|&i, &j| (sgn*cross(last, p[i], p[j])).cmp(&0))
                      .unwrap();
        taken[i] = true;
        last = p[i];
        print!("{} ", i + 1);
    }
    println!("");
}

fn main() {
    std::thread::Builder::new().stack_size(50 << 20)
        .spawn(maint).unwrap().join().unwrap();
}
