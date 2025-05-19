use std::{io::{self, BufRead}, process::exit};

const MOD: i32 = 1000000007;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<char> = line.trim().chars().collect();
    let n = nums.len();
    // 前i个字符的合法字符串数量。
    let mut f = vec![0; n + 1];
    f[0] = 1;
    f[1] = 1;
    for x in &nums {
        if *x == 'w' || *x == 'm' {
            println!("{}", 0);
            exit(0);
        }
    }
    for i in 2..n + 1 {
        f[i] = f[i - 1];
        if nums[i - 1] == nums[i - 2] && (nums[i - 1] == 'u' || nums[i - 1] == 'n') {
            f[i] = (f[i] + f[i - 2]) % MOD;
        }
    }
    // println!("{}", MOD);
    println!("{}", f[n]);
}