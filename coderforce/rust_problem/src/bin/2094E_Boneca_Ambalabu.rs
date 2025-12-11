use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line: String = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0];
        let nums = input_nums();
        let mut cnt = vec![0; 31];
        for i in (0..=30).rev() {
            for x in &nums {
                if (x >> i) & 1 == 1 {
                    cnt[i] += 1;
                }
            }
        }
        let mut total = 0;
        for x in &nums {
            let mut res = 0;
            for i in 0..=30 {
                if (x >> i) & 1 == 1 {
                    res += 2usize.pow(i) * (n as usize - cnt[i as usize]);
                } else {
                    res += 2usize.pow(i) * cnt[i as usize];
                }
            }
            total = total.max(res);
        }
        ans.push(total);
    }
    for x in ans {
        println!("{}", x);
    }
}

