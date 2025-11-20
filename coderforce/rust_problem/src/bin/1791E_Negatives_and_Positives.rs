use std::{i64, io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let t: i64 = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let _n: i64 = input_nums()[0];
        let nums = input_nums();
        let mut cnt = 0;
        let mut zero = false;
        let mut sum = 0;
        let mut m = i64::MAX;
        for x in nums {
            if x == 0 {
                zero = true;
            }
            if x < 0 {
                cnt += 1;
            }
            m = m.min(x.abs());
            sum += x.abs();
        }
        if zero || cnt % 2 == 0 {
            ans.push(sum);
        } else {
            ans.push(sum - m * 2);
        }
    }
    for x in ans {
        println!("{}", x);
    }
}


