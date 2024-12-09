use std::{i64, io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n = nums[0];
        let a = input_nums();
        let b = input_nums();
        let mut sa: Vec<i64> = vec![0; n as usize + 1];
        let mut sb: Vec<i64> = vec![0; n as usize + 1];
        for i in 0..n {
            let i = i as usize;
            sa[i + 1] = sa[i] + a[i];
            sb[i + 1] = sb[i] + b[i];
        }
        let mut res = i64::MAX;
        for i in 0..n {
            let i = i as usize;
            let pre = sa[n as usize] - sa[i + 1];
            let suf = sb[i] - sb[0];
            res = res.min(pre.max(suf));
        }
        ans.push(res);
    }
    for x in ans {
        println!("{x}");
    }
}


