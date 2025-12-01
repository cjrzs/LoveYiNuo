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
        let n = input_nums()[0] as usize;
        let mut p = vec![0; n + 1];
        for i in 0..=n {
            p[i] = i;
        }
        let mut res = Vec::new();
        let nums = input_nums();
        let mut mn = nums[0];
        let mut buffer = Vec::new();
        for i in 1..n {
            if nums[i] > mn {
                res.push((mn, nums[i]));
                while buffer.len() > 0 {
                    let k = buffer.pop().unwrap();
                    res.push((nums[i], k));
                    mn = mn.min(k);
                }
            } else {
                buffer.push(nums[i]);
            }
        }
        if buffer.len() > 0 {
            ans.push(("NO", Vec::new()));
        } else {
            ans.push(("YES", res));
        }
    }
    for (x, res) in ans {
        println!("{}", x);
        if x == "YES" {
            for x in res {
                println!("{} {}", x.0, x.1);
            }
        }
    }
}

