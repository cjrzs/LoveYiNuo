use std::{io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let mut res = 0;
        let nums = input_nums();
        let n = nums[0];
        for i in 0..n {
            for j in 0..n {
                if i * 2 + j * 4 == n {
                    res += 1;
                }
            }
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}
