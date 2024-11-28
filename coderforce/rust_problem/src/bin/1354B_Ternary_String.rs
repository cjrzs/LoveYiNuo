use std::io::Stdin;
use std::io::{self, BufRead};
use std::cmp::{max, min};

fn input(stdin: Stdin) -> String {
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

fn main() {
    let stdin = io::stdin();
    let n: i32 = input(stdin).trim().parse().unwrap();
    let mut ans: Vec<i32> = Vec::new();
    for _ in 0..n {
        let stdin = io::stdin();
        let line = input(stdin);
        let new_line = line.trim();
        let mut one: i32 = -1;
        let mut two: i32 = -1;
        let mut three: i32 = -1;
        let mut res = i32::MAX;
        // println!("line: {}", line);
        for (i, x) in new_line.chars().enumerate() {
            if x == '1' {
                one = i as i32;
            } else if  x == '2' {
                two = i as i32;
            } else {
                three = i as i32;
            }
            // println!("qqq: {} {} {} {} {}", i, one, two, three, x);
            if one >= 0 && two >= 0 && three >= 0 {
                let t1: i32 = max(max(one, two), three);
                let t2: i32 = min(min(one, two), three);
                // println!("max: {}, min: {}", t1, t2);
                res = min(res, t1 - t2 + 1);
            }
            // println!("res: {}", res);
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", if x != i32::MAX {x} else {0});
    }
}

