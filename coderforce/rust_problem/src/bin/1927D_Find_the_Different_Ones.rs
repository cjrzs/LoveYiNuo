use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n = nums[0];
        let arr = input_nums();
        let mut p: Vec<i64> = vec![-1; n as usize + 1];
        for i in 1..n {
            let i = i as usize;
            if arr[i as usize] == arr[i - 1] {
                p[i] = p[i - 1];
            } else {
                p[i] = i as i64 - 1;
            }
        }
        // println!("{:?}", p);
        let nums = input_nums();
        let m = nums[0];
        for _ in 0..m {
            let nums = input_nums();
            let [l, r] = [nums[0], nums[1]];
            // println!("l: {} r: {} {}, {}", l, r, p[r as usize - 1], l);
            if p[r as usize - 1] + 1 >= l {
                ans.push((p[r as usize - 1] + 1, r));
            } else {
                ans.push((-1, -1));
            }
        }
        ans.push((-2, -2));
    }
    for (l, r) in ans {
        if l != -2 {
            println!("{} {}", l, r);
        } else {
            println!();
        }
        
    }
}


