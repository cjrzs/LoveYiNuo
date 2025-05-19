use std::io::{self, BufRead};
use std::cmp::max;

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let _n = nums[0];
    let nums = input_nums();
    let mut maxx = 0;
    let mut d = vec![0; 100005];
    for x in nums {
        d[x as usize] += 1;
        maxx = maxx.max(x);
    }
    let mut f = vec![0; 100005];
    f[1] = d[1];
    for i in 2..=maxx {
        f[i as usize] = max(f[i as usize - 1], f[i as usize - 2] + d[i as usize] * i);
    }
    println!("{}", f[maxx as usize]);
}

