use std::io::{self, BufRead};

fn input_nums() -> Vec<i32> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let _n = nums[0];
    let nums = input_nums();
    let mut ans = Vec::new();
    for x in nums {
        ans.push(to_32768(x));
    }
    let formatted = ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
    println!("{}", formatted);
}

fn to_32768(x: i32) -> i32 {
    let mut res = 20;
    for i in 0..15 {
        for j in 0..15 {
            if (x + i) * 2_i32.pow(j) % 32768 == 0 {
                res = std::cmp::min(res, i + j  as i32);
            }
        }
    }
    res
}
