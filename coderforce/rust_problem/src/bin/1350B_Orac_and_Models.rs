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
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n: usize = nums[0] as usize;
        let nums = input_nums();
        let mut f = vec![1; n + 1];
        let mut res = 0;
        for i in 1..n + 1 {
            for j in (i * 2..=n).step_by(i) {
                if nums[j - 1] > nums[i - 1] {
                    f[j] = std::cmp::max(f[i] + 1, f[j]);
                }
            }
        }
        for x in f {
            res = std::cmp::max(res, x);
        }
        ans.push(res);
    }
    for x in ans {
        println!("{x}");
    }
}


