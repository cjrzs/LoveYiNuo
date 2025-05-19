use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums = input_nums();
    let t = nums.parse::<i64>().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums: Vec<char> = input_nums().chars().collect();
        let n = nums.len();
        let mut f = vec![vec![0; 2]; n + 1];
        for i in 1..n + 1 {
            if nums[i - 1] == '0' {
                f[i][0] = f[i - 1][1] + 1;
            } else if nums[i - 1] == '1' {
                f[i][1] = f[i - 1][0] + 1;
            } else {
                f[i][0] = f[i - 1][1] + 1;
                f[i][1] = f[i - 1][0] + 1;
            }
        }
        let mut res: i64 = 0;
        for i in 1..n + 1 {
            res += std::cmp::max(f[i][0], f[i][1]);
        }
        ans.push(res);
    }
    for x in ans {
        println!("{x}");
    }
}