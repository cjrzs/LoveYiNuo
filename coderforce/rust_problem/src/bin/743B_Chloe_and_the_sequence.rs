use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line: String = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn func(n: i64, k: i64) -> i64 {
    let t = 2i64.pow(n as u32 - 1);
    if k == t {
        return n;
    } else if k < t {
        return func(n - 1, k);
    } else {
        return func(n - 1, k - t);
    }
}

fn main() {
    let nums = input_nums();
    let (n, k)= (nums[0], nums[1]);
    println!("{}", func(n, k));
}
