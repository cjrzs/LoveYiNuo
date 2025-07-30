fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut res = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let (n, p, k) = (nums[0], nums[1], nums[2]);
        let mut nums = input_nums();
        nums.sort();
        let mut ans = 0;
        let mut dp = vec![i64::MAX; n as usize + 1];
        dp[1] = nums[0];
        dp[0] = 0;
        for i in 2..n + 1 {
            if i >= k {
                dp[i as usize] = dp[i as usize - 1].min(dp[i as usize - k as usize]) + nums[i as usize - 1];
            } else {
                dp[i as usize] = dp[i as usize - 1] + nums[i as usize - 1];
            }
        }
        // println!("{:?}", dp);
        for i in 0..dp.len() {
            if dp[i] <= p {
                ans = i;
            }
        }
        res.push(ans);
    }
    for i in res {
        println!("{}", i);
    }
}

