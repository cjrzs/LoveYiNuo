use std::io::{self, BufRead};
use std::cmp::min;

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let [n, m] = [nums[0], nums[1]];
        let mut d = vec![n; n + 1];
        for _ in 0..m {
            let nums = input_nums();
            let [mut a, mut b] = [nums[0], nums[1]];
            if b < a {
                std::mem::swap(&mut a, &mut b);
            }
            d[a] = min(d[a], b - 1);
        }
        for i in (1..n - 1).rev() {
            d[i] = min(d[i], d[i + 1]);
        }
        let mut res = 0;
        for i in 1..n + 1 {
            res += d[i] - i + 1;
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}
