use std::{collections::HashSet, io::{self, BufRead}};

fn input_nums() -> Vec<i32> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().chars().map(|x| x.to_digit(10).unwrap() as i32).collect()
}

fn dfs(u: usize, num: usize, nums: &Vec<i32>, n: usize, set: &mut HashSet<usize>, cur: usize) -> (bool, usize) {
    // println!("{}", num);
    if u > 0 && num % 8 == 0 {
        return (true, num)
    }
    if u >= 3 {
        return if num % 8 == 0 {
            (true, num)
        } else {
            (false, num)
        }
    }
    for i in cur..n {
        if !set.contains(&i) {
            set.insert(i);
            let (found, result) = dfs(u + 1, num * 10 + nums[i] as usize, nums, n, set, i);
            if found {
                return (true, result);
            }
            set.remove(&i);
        } else {
            continue;
        }
    }
    (false, num)
}

fn main() {
    let nums = input_nums();
    let n = nums.len();
    let mut set: HashSet<usize> = HashSet::new();
    let (flag, num) = dfs(0, 0, &nums, n, &mut set, 0);
    if flag {
        println!("YES");
        println!("{}", num);
    } else {
        println!("NO");
    }
}