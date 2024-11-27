use std::i64;
use::std::io::{self, BufRead};

fn input_nunms() -> Vec<i64> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nunms();
    let mut ans = Vec::new();
    for _ in 0..nums[0] {
        let nums = input_nunms();
        // println!("{:?}", nums);
        let _n = nums[0];
        let nums = input_nunms();
        // println!("{:?}", nums);
        let mut res:i64 = 0;
        let mut f: bool = true;
        let mut t: i64 = 0;
        for (i, x) in nums.iter().enumerate() {
            if i == 0 {
                t = *x;
                f = if *x > 0 {true} else {false};
            } else {
                if (f && *x > 0) || (!f && *x < 0) {
                    t = t.max(*x);
                } else {
                    res += t;
                    t = *x;
                    f = !f;
                }
            }  
        }
        ans.push(res + t);
        // println!("{}", res);
    }
    for x in ans {
        println!("{}", x);
    }
}


