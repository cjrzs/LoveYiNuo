use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
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
        let [n, _q] = [nums[0], nums[1]];
        let nums = input_nums();
        // f[i][0] 0~i个元素中选择减去第i个得到的最大值。
        // f[i][1] 0~i个元素中选择加上第i个得到的最大值。
        let mut f = vec![vec![0; 2]; (n + 1).try_into().unwrap()];
        f[0][1] = nums[0];
        for i in 1..n {
            let i = i as usize;
            f[i][0] = std::cmp::max(f[i - 1][0], f[i - 1][1] - nums[i]);
            f[i][1] = std::cmp::max(f[i - 1][1], f[i - 1][0] + nums[i]);
        }
        let res = std::cmp::max(f[n as usize - 1][0], f[n as usize - 1][1]);
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}



