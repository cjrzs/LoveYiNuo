use std::io::{self, BufRead};

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let [n, k] = [nums[0] as usize, nums[1] as usize];
        let nums = input_nums();
        let mut len = 1;
        let mut res = 0;
        for i in 1..n {
            if nums[i] * 2 > nums[i - 1] {
                len += 1;
                if len > k + 1 {
                    len = k + 1;
                }
                if len == k + 1 {
                    res += 1;
                }
            } else {
                len = 1;
            }
        }
        ans.push(res);
    }
    for x in ans {
        println!("{x}");
    }
}

