use std::{io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    let r#mod = 1_000_000_007;
    for _ in 0..t {
        let nums = input_nums();
        let (n, k) = (nums[0], nums[1]);
        let mut res = 1;
        for _ in 0..k {
            res *= n;
            res %= r#mod;
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}


