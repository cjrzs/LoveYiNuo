use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n: usize = nums[0].try_into().unwrap();
        let mut nums = input_nums();
        nums.sort();
        let a1: i64 = nums[n - 1] * nums[n - 2] * nums[n - 3] * nums[n - 4] * nums[n - 5];
        let a2: i64 = nums[n - 1] * nums[0] * nums[1] * nums[2] * nums[3];
        let a3: i64 = nums[n - 1] * nums[n - 2] * nums[n - 3] * nums[0] * nums[1];
        ans.push(a1.max(a2).max(a3));
    }
    for x in ans {
        println!("{x}");
    }
}

