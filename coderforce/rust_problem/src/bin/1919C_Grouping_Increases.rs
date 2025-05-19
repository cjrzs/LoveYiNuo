use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n = nums[0] as usize;
        let nums = input_nums();
        let mut first = n as i64 + 1;
        let mut secend = n as i64 + 1;
        let mut res = 0;
        for i in 0..n {
            if nums[i] <= first && nums[i] <= secend {
                if first > secend {
                    secend = nums[i];
                } else {
                    first = nums[i];
                }
            } else if nums[i] <= first {
                first = nums[i];
            } else if nums[i] <= secend {
                secend = nums[i];
            } else {
                res += 1;
                if first > secend {
                    secend = nums[i];
                } else {
                    first = nums[i];
                }
            }
        }
        ans.push(res);
    }
    for x in ans {
        println!("{x}");
    }
}


