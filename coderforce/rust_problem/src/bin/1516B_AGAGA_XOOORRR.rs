use std::{io::{self, BufRead}};

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0];
        let nums = input_nums();
        let mut a = 0;
        for i in 0..n {
            a = a ^ nums[i];
        }
        if a == 0 {
            ans.push("YES");
            continue;
        }
        let mut b = 0;
        let mut cnt = 0;
        let mut broken = false;
        for i in 0..n {
            b = b ^ nums[i];
            if b == a {
                cnt += 1;
                b = 0;
            }
            if cnt >= 2 {
                ans.push("YES");
                broken = true;
                break;
            }
        }
        if !broken {
            ans.push("NO");
        }
    }
    for x in ans {
        println!("{}", x);
    }
}