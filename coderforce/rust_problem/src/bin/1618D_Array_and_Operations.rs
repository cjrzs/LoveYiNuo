use std::io::{self, BufRead};

fn input_nums() -> Vec<i32> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans: Vec<i32> = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let [n, m] = [nums[0], nums[1]];
        let mut nums = input_nums();
        nums.sort();
        // println!("{:?}", nums);
        let mut res = (&nums[0..n as usize - 2 * m as usize]).iter().sum();
        // println!("init res: {}", res);
        for i in n - m * 2 .. n - m {
            // println!("smaill: {} big: {}", nums[i as usize], nums[i as usize + m as usize]);
            res += nums[i as usize] / nums[i as usize + m as usize];
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}


