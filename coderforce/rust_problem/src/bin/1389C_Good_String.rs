use std::io::{self, BufRead};

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

fn main() {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    let t = line.trim().parse().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let mut res_single = 0;
        let mut res_double = 0;
        let nums = input_nums();
        for c in 0..10 {
            let mut k = 0;
            for x in nums.clone() {
                if x == c {
                    k += 1;
                }
            }
            res_single = res_single.max(k);
            for d in 0..10 {
                if c == d {
                    continue;
                }
                let mut k = 0;
                let mut flag = 0;
                let q = [c, d];
                for x in nums.clone() {
                    if x == q[flag] {
                        k += 1;
                        flag = 1 - flag;
                    }
                }
                if k % 2 != 0 {
                    k -= 1;
                }
                res_double = res_double.max(k);
                // println!("{} {} {}", c, d, k);
            }
        }
        // println!("{} {}", res_single, res_double);
        ans.push(nums.len() - res_single.max(res_double));
    }
    for a in ans {
        println!("{}", a);
    }
}


