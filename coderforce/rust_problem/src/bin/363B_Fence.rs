use std::{i64, io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let (n, k) = (nums[0] as usize, nums[1] as usize);
    let nums = input_nums();
    let mut j: usize = 0;
    let mut res = i64::MAX;
    let mut ans = 0;
    let mut cur = 0;
    for i in 0..n {
        cur += nums[i as usize];
        if i - j + 1 > k {
            cur -= nums[j];
            j += 1;
        }
        if i - j + 1 == k && cur < res {
            res = cur;
            ans = j + 1;
        }
    }
    println!("{}", ans);
}

