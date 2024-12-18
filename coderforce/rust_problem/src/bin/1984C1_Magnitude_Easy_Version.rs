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
        let mut res: i64 = 0;
        let mut minn: i64 = 0;
        for i in 0..n {
            let i = i as usize;
            res += nums[i];
            minn = minn.min(res);
        }
        // println!("{}, {}", res, minn);
        ans.push(res - 2 * minn);
    }
    for x in ans {
        println!("{x}");
    }
}

