use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums = input_nums();
    let t = nums.parse::<i64>().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let nums: Vec<i64> = nums.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let (n, m, k) = (nums[0], nums[1], nums[2]);
        let nums: Vec<char> = input_nums().chars().collect();
        let mut f = vec![-1; n as usize + 2];
        f[0] = k;
        for i in 1..n + 2 {
            let i = i as usize;
            if i != n as usize + 1 && nums[i - 1] == 'C' {
                continue;
            }
            for j in 1..m + 1 {
                let j = j as usize;
                if i >= j && (i == j || nums[i - j - 1] == 'L') {
                    f[i] = f[i].max(f[i - j]);
                }
            }
            if i > 1 && nums[i - 2] == 'W' {
                f[i] = (f[i]).max(f[i - 1] - 1);
            }
        }
        // println!("{:?}", f);
        ans.push(f[n as usize + 1] >= 0);
    }
    for x in ans {
        println!("{}", if x {"YES"} else {"NO"});
    }
}
