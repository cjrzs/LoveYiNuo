use std::{io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn get_best_bit(x: i64) -> i64 {
    let mut t = x;
    let mut res = 0;
    while t > 0 {
        t = t >> 1;
        res += 1;
    }
    res
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let _n = input_nums()[0];
        let nums = input_nums();
        let mut d = vec![Vec::new(); 31];
        let mut res = 0;
        for x in nums {
            d[get_best_bit(x) as usize].push(x);
        }
        for xv in d {
            let k = xv.len() as i64;
            if k > 0 {
                res += k * (k - 1) / 2;
            }
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}


