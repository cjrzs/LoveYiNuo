use std::{io::{self, BufRead}, usize};

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans: Vec<usize> = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let _n = nums[0];
        let mut nums = input_nums();
        nums.sort();
        let mut t: usize = 0;
        let mut res: usize = 0;
        for x in nums {
            t += 1;
            // println!("{x}, {t}");
            if x <= t {
                res += 1;
                t = 0;
            }
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}


