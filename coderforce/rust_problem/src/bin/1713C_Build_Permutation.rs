use std::io::{self, BufRead};

fn input_nums() -> i64 {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().parse::<i64>().unwrap()
}

fn main() {
    let t = input_nums();
    let mut ans = Vec::new();
     
    for _ in 0..t {
        let n = input_nums();
        let mut i = n - 1;
        let mut res = vec![0; n as usize];
        while i >= 0 {
            // 找到最小的平方数 >= i
            let s = ((i as f64).sqrt().ceil() as i64).pow(2);
            let start = s - i;
            if start < 0 {
                break;
            }

            // 反转区间 [start, i]
            for j in start..=i {
                res[j as usize] = s - j;
            }

            if start == 0 {
                break;
            }
            i = start - 1;
        }

        ans.push(res);
    }
    for res in ans {
        println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}
