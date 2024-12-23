use std::io::{self, BufRead};

const MOD: i64 = 1000000007;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<i64> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let [n, k] = [nums[0] as usize, nums[1] as usize];
    let mut f = vec![vec![0; n + 1]; k + 1];
    for i in 1..=n { f[1][i] = 1 };
    for i in 2..=k {
        for j in 1..=n {
            let mut d = 1;
            while d * j <= n {
                f[i][j] = (f[i - 1][j * d] + f[i][j]) % MOD;
                d += 1;
            }
        }
    }
    let mut res = 0;
    for i in 1..=n {
        res = (res + f[k][i]) % MOD;
    }
    println!("{res}");
}


