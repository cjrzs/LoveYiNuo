use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let t = line.trim().parse().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let n: i32 = line.trim().parse().unwrap();
        let mut nums: Vec<Vec<char>> = Vec::new();
        for _ in 0..n {
            let mut line = String::new();
            stdin.lock().read_line(&mut line).unwrap();
            let tmp = line.chars().collect();
            nums.push(tmp);
        }
        let mut flag: bool = true;
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                // println!("{} {} {}", i, j, nums[i as usize][j as usize]);
                if nums[i as usize][j as usize] == '1' {
                    if i != n - 1 && j != n - 1 && (i + 1 < n && nums[i as usize + 1][j as usize] != '1') && (j + 1 < n && nums[i as usize][j as usize + 1] != '1') {
                        flag = false;
                    }
                }
            }
        }
        ans.push(if flag {"YES"} else {"NO"});
    }
    for x in ans {
        println!("{}", x);
    }
}


