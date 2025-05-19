use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let t = input_nums();
    let t = t.parse::<i32>().unwrap();
    let mut ans = Vec::new();
    for _q in 0..t {
        let n = input_nums();
        let n = n.parse::<i32>().unwrap();
        let nums = input_nums();
        let s: Vec<i32> = nums.chars()
                        .map(|x| x.to_string().parse::<i32>().unwrap())
                        .collect();
        if n == 2 {
            ans.push(nums.parse().unwrap());
        } else if n==3&&s[0]!=0&&s[1]==0&&s[2]!=0 {
            ans.push(if s[0]==1||s[2]==1 {(s[0])*(s[2])} else {(s[0])+(s[2])});
        } else {
            let nums: Vec<char> = nums.chars().collect();
            let nums: Vec<i32> = nums
                        .iter()
                        .map(|x| x.to_string().parse::<i32>().unwrap())
                        .collect();
            let mut flag = true;
            for x in 0..n {
                if nums[x as usize] == 0 {
                    ans.push(0);
                    flag = false;
                    break;
                }
            }
            if !flag {
                continue;
            }
            let mut res = i32::MAX;
            for i in 1..n {
                let mut total = 0;
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    let mut k = nums[j as usize];
                    if j == i - 1 {
                        k = nums[j as usize] * 10 + nums[i as usize];
                    }
                    if k > 1 {
                        total += k;
                    }
                }
                res = res.min(total);
            }
            ans.push(res);
        }
    }
    for x in ans {
        println!("{x}");
    }
}


