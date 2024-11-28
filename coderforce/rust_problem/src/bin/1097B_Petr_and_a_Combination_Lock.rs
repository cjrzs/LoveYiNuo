use std::io::{self, BufRead};

fn input_nums() -> i32 {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let n = line.trim().parse().unwrap();
    n
}

fn dfs(nums: &Vec<i32>, u: usize, val: i32) -> bool {
    if u == 0 {
        return val == 0;
    }
    if dfs(nums, u - 1, (val + nums[u - 1]) % 360) {
        return true;
    }
    if dfs(nums, u - 1, (val - nums[u - 1]) % 360) {
        return true;
    }
    return false;
}

fn main() {
    let n = input_nums();
    let mut nums = Vec::new();
    for _ in 0..n as usize {
        let x = input_nums();
        nums.push(x);
    }
    if dfs(&nums, n as usize, 0) {
        println!("YES");
    } else {
        println!("NO");
    }
}