use std::{collections::HashSet, io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let _ = input_nums()[0];
        let nums = input_nums();
        let mut d = HashSet::new();
        for x in &nums {
            d.insert(x);
        }
        let mut flag2 = false;
        for i in 1..=1024 {
            let mut flag = true;
            for x in &nums {
                if !d.contains(&(x ^ i)) {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans.push(i);
                flag2 = true;
                break;
            }
        }
        if !flag2 {
            ans.push(-1);
        }
    }
    for x in ans {
        println!("{}", x);
    }
}

