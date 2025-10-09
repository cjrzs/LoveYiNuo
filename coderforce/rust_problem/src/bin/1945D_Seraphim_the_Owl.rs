use std::io::BufRead;

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let (n, m) = (nums[0] as usize, nums[1] as usize);
        let a = input_nums();
        let b = input_nums();
        let mut pre = vec![0; b.len() + 1];
        for i in 1..b.len() + 1 {
            pre[i] = pre[i - 1] + b[i - 1];
        }
        // println!("{:?}", pre);
        let mut res = 0;
        let mut j = n;
        for i in (m..n).rev() {
            if a[i] < b[i] {
                res += a[i];
                // println!("{}:  {}", i, pre[j] - pre[i + 1]);
                if j >= m {
                    res += pre[j] - pre[i + 1];
                }
                j = i;
            }
        }
        // println!("{}", res);
        if j >= m {
            let mut mini = i64::MAX;
            for i in 0..m {
                mini = mini.min(a[i] + pre[j] - pre[i + 1]);
            }
            res += mini;
        }
        ans.push(res);
    }
    for i in ans {
        println!("{}", i);
    }
}



