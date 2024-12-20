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
    let n: usize = nums[0] as usize;
    let nums1 = input_nums();
    let nums2 = input_nums();
    let mut f = vec![vec![0; 3]; n + 1];
    for i in 1..n + 1 {
        f[i][0] = max(f[i - 1][0], max(f[i - 1][1], f[i - 1][2]));
        f[i][1] = max(f[i - 1][2], f[i - 1][0]) + nums1[i - 1];
        f[i][2] = max(f[i - 1][1], f[i - 1][0]) + nums2[i - 1];
    }
    println!("{}", max(f[n][0], max(f[n][1], f[n][2])));
}
                                                                                       


