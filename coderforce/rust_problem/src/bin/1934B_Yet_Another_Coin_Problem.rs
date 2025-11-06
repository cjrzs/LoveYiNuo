use std::{i64, io::{self, BufRead}};

// fn dfs(n: i64, cnt: i64, sum: i64, res: &mut i64, nums: &[i64; 5], mem: &mut HashMap<i64, i64>) {
//     if sum > n || cnt >= *res {
//         return;
//     }
//     if let Some(&best) = mem.get(&sum) {
//         if best <= cnt {
//             return;
//         }
//     }
//     if sum == n {
//         *res = cmp::min(*res, cnt);
//     }
//     for &i in nums {
//         dfs(n, cnt + 1, sum + i, res, nums, mem);
//     }
// }

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();
    let t: i64 = it.next().unwrap().unwrap().trim().parse().unwrap();
    let mut ans = Vec::new();
    let coins = [1, 3, 6, 10, 15];
    let mut dp = vec![i64::MAX / 4; 30];
    dp[0] = 0;
    for i in 1..30 {
        for &c in &coins {
            if i >= c {
                dp[i] = dp[i].min(dp[i - c] + 1);
            }
        }
    }
    // println!("{}", t);
    for _ in 0..t {
        let n: i64 = it.next().unwrap().unwrap().trim().parse().unwrap();
        let base = n / 15;
        let rem = (n % 15) as usize;

        // 情况1：直接用 base 个 15 + dp[rem]
        let mut res = base + dp[rem];

        // 情况2：少用一个 15（前提 n >= 15）
        if n >= 15 {
            let rem2 = (rem + 15) as usize;
            res = res.min(base - 1 + dp[rem2]);
        }
        ans.push(res);
    }
    // println!("{:?}", ans);
    for x in ans {
        println!("{}", x);
    }
}




