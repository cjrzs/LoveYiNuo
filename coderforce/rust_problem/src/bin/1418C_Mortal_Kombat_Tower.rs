use std::io::{self, BufRead};

const INF: usize = 1 << 30;

fn input_nums() -> Vec<usize> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n = nums[0];
        let a = input_nums();
        let mut dp = vec![vec![INF; 2]; n + 2];
        // dp[i][0]：打完前 i 个 boss，下一回合轮到你
        // dp[i][1]：打完前 i 个 boss，下一回合轮到你朋友
        dp[0][1] = 0;
        for i in 0..=n {
            // 如果当前轮到你（dp[i][0]），你打 1 或 2 个 boss
            if i < n {
                dp[i + 1][1] = dp[i + 1][1].min(dp[i][0]);
            }
            if i + 1 < n {
                dp[i + 2][1] = dp[i + 2][1].min(dp[i][0]);
            }

            // 如果当前轮到朋友（dp[i][1]），他打 1 或 2 个 boss（每个 hard 要 skip 点）
            if i < n {
                dp[i + 1][0] = dp[i + 1][0].min(dp[i][1] + a[i]);
            }
            if i + 1 < n {
                dp[i + 2][0] = dp[i + 2][0].min(dp[i][1] + a[i] + a[i + 1]);
            }
        }
        ans.push(dp[n][0].min(dp[n][1]));
    }
    for x in ans {
        println!("{}", x);
    }
}