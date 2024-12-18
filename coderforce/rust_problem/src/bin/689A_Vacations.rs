use std::io::{self, BufRead};
use std::cmp::min;

fn input_nums() -> Vec<i32> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let n: usize = nums[0] as usize;
    let nums = input_nums();
    let mut f = vec![vec![n; 3]; n + 1];

    f[0][0] = 0;
    f[0][1] = 0;
    f[0][2] = 0;
    for i in 1..=n {
        f[i][0] = min(f[i - 1][0], min(f[i - 1][1], f[i - 1][2])) + 1;
        if nums[i - 1] == 1 || nums[i - 1] == 3 {
            f[i][1] = min(f[i - 1][0], f[i - 1][2]);
        }
        if nums[i - 1] == 2 || nums[i - 1] == 3 {
            f[i][2] = min(f[i - 1][0], f[i - 1][1]);
        }
    }
    println!("{}", min(f[n][0], min(f[n][1], f[n][2])));
}


