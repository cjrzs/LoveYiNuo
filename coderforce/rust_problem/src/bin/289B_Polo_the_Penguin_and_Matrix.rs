use std::{io::{self, BufRead}, process::exit};

fn input_nums() -> Vec<i32> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let [n, m, d] = [nums[0], nums[1], nums[2]];
    let mut nums: Vec<i32> = Vec::new();
    for _ in 0..n {
        let nums2 = input_nums();
        for x in nums2 {
            nums.push(x);
        }
    }
    let t = nums[0] % d;
    nums.sort();
    let base = (n * m) / 2;
    let mut res = 0;
    for x in &nums {
        if x % d != t {
            println!("{}", -1);
            exit(0);
        }
        res += (x - nums[base as usize]).abs() / d;
    }
    println!("{}", res);
}
    