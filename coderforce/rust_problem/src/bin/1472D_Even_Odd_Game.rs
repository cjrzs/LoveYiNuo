use std::io::{self, BufRead};

fn input_nunms() -> Vec<usize> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nunms();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nunms();
        let _n = nums[0];
        let mut nums = input_nunms();
        nums.sort_by(|a, b| b.cmp(a));
        let mut alice: usize = 0;
        let mut bob: usize = 0;
        for (i, x) in nums.iter().enumerate() {
            if i % 2 == 0 {
                alice += if x % 2 == 0 {*x} else {0};
            } else {
                bob += if x % 2 == 1 {*x} else {0};
            }
        }
        let mut res = "";
        if alice > bob {
            res = "Alice";
        } else if alice == bob {
            res = "Tie";
        } else {
            res = "Bob";
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}

