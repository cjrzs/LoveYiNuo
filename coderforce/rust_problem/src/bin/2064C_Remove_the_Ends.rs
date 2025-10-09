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
        let n = input_nums()[0] as usize;
        let nums = input_nums();
        let mut sum = vec![0; n + 1];
        let mut suf = vec![0; n + 1];
        for i in 0..n {
            if i == 0 {
                if nums[i] > 0 {
                    sum[i] = nums[i];
                }
                continue;
            } 
            if nums[i] > 0 {
                sum[i] = sum[i - 1] + nums[i];
            } else {
                sum[i] = sum[i - 1];
            }
        }
        // println!("sum: {:?}", sum);
        for i in (0..n).rev() {
            if i == n - 1 {
                if nums[i] < 0 {
                    suf[i] = nums[i].abs();
                }
                continue;
            }
            if nums[i] < 0 {
                suf[i] = suf[i + 1] + nums[i].abs();
            } else {
                suf[i] = suf[i + 1];
            }
        }
        // println!("suf: {:?}", suf);
        let mut res = 0;
        for i in 0..n {
            res = res.max(sum[i] + suf[i]);
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}

