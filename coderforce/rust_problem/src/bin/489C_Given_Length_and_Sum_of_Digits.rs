use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let [m, s] = [nums[0], nums[1]];
    if s == 0 && m == 1 {
        println!("{} {}", 0, 0);
    } else if s == 0 && m != 1 {
        println!("{} {}", -1, -1);
    } else {
        let mut sum1 = s;
        let mut sum2 = s;
        let mut minn = String::from("");
        let mut maxx = String::from("");
        let mut flag = false;
        for i in 1..=m  {
            for j in 0..10 {
                if i == 1 && j == 0 {
                    continue;
                }
                if check(m - i, sum1 - j) {
                    sum1 -= j;
                    minn += &j.to_string();
                    break;
                }
            }
            for j in (0..=9).rev() {
                if i == 1 && j == 0 {
                    continue;
                }
                if sum2 >= j {
                    sum2 -= j;
                    maxx += &j.to_string();
                    break;
                }
            }
            // println!("sum2: {}", sum2);
            if i == m && sum2 <= 0 {
                flag = true;
            }
        }
        if !flag {
            maxx = "-1".to_string();
        }
        if minn == "" {
            minn = "-1".to_string();
        }
        println!("{} {}", minn, maxx);
    }
}

fn check(m: i32, s: i32) -> bool {
    return s >= 0 && m * 9 >= s;
}

