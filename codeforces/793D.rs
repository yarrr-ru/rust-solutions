#[allow(unused_imports)]
use std::cmp::{min,max};
const INF: i32 = 0x3f3f3f3f;

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

const NMAX: usize = 80;

fn main1() {
    let mut c = [[INF; NMAX]; NMAX];
    let mut dp = [[[INF; NMAX]; NMAX]; NMAX];
    dp[0] = [[0; NMAX]; NMAX];
    let mut scan = Scanner::new();
    let n = scan.next::<usize>();
    let k = scan.next::<usize>();
    let m = scan.next::<usize>();
    for _ in 0..m {
        let u = scan.next::<usize>() - 1;
        let v = scan.next::<usize>() - 1;
        let w = scan.next::<i32>();
        c[u][v] = min(c[u][v], w);
    }
    for s in 1..k {
        for i in 0..n {
            for j in 0..n {
                let dir: i32 = if j > i { 1 } else { -1 };
                let mut ii = i;
                while ii != j {
                    ii = (ii as i32 + dir) as usize;
                    let jj = (i as i32 + dir) as usize;
                    let val = min(dp[s-1][ii][j], dp[s-1][ii][jj]);
                    dp[s][i][j] = min(dp[s][i][j], c[i][ii] + val);
                }
            }
        }
    }
    let mut ans = INF;
    for i in 0..n {
        ans = min(ans, min(dp[k-1][i][0], dp[k-1][i][n-1]));
    }
    if ans == INF {
        ans = -1;
    }
    println!("{}", ans);
}

fn main() {
    std::thread::Builder::new().stack_size(1 << 26)
        .spawn(main1).unwrap().join().unwrap();
}
