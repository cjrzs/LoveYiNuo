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
        let _n = nums[0];
        let nums = input_nums();
        let mut res = 0;
        let mut nums_new = Vec::new();
        for (i, &x) in nums.iter().enumerate() {
            let idx = i as i64 + 1;
            if idx > x {
                nums_new.push((x, i + 1));
            }
        }
        // println!("{:?}", nums_new);
        let m: i64 = nums_new.len() as i64;
        if m < 2 {
            ans.push(0);
        } else {
            for (x, _i) in nums_new.iter() {
                let mut l: i64 = 0;
                let mut r: i64 = m;
                while l < r {
                    let mid: i64 = (l + r + 1) >> 1;
                    if *x > nums_new[mid as usize].1 as i64{
                        l = mid;
                    } else {
                        r = mid - 1;
                    }
                }
                // println!("r: {r}");
                if r == 0 {
                    if *x > nums_new[0].1 as i64 {
                        res += 1;
                    }
                } else {
                    res += r + 1;
                }
                // res += m - r + 1
            }
            ans.push(res);
        }
    }
    for x in ans {
        println!("{x}");
    }
}


