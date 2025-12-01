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
        let n = input_nums()[0];
        let mut nums = input_nums();
        nums.sort();
        let mut res = true;
        if n == 1 {
            if nums[0] != 1 {
                res = false;
            }
        } else {
            let mut mn = nums[0];
            if mn != 1 {
                res = false;
            }
            for i in 1..n {
                if nums[i as usize] > mn {
                    res = false;
                    break;
                }
                mn += nums[i as usize];
            }
        }
        ans.push(if res { "YES" } else { "NO" });
    }
    for x in ans {
        println!("{}", x);
    }
}



