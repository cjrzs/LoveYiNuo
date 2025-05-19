use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums = input_nums();
    let nums: Vec<i64> = nums.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let nums: Vec<i64> = nums.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let [n, k] = [nums[0], nums[1]];
        let a: Vec<char> = input_nums().chars().collect();
        let b: Vec<char> = input_nums().chars().collect();
        let mut f = vec![vec![0; 26]; 2];
        for i in 0..n {
            let x1 = a[i as usize] as u8 - b'a';
            f[0][x1 as usize] += 1;
            let x2 = b[i as usize] as u8 - b'a';
            f[1][x2 as usize] += 1;
        }
        let mut flag = true;
        for i in 0..26 {
            let t = f[0][i] - f[1][i];
            if t < 0 {
                flag = false;
                break;
            } else if t == 0 {
                continue;
            } else if t % k != 0 {
                flag = false;
                break;
            } else {
                if i < 25 {
                    f[0][i + 1] += t;
                } else {
                    flag = false;
                }
            }
        }
        ans.push(if flag {"Yes"} else {"No"});
    }
    for x in ans {
        println!("{x}");
    }
}


