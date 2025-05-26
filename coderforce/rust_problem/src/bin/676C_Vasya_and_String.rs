use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let nums: Vec<usize> = lines.next().unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, k) = (nums[0], nums[1]);
    let s: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut l = 0;
    let mut r = 0;
    let mut res = 0;
    let mut b = 0;
    while l < n && r < n {
        // println!("11 {} {}", l, r);
        if s[r] == 'b' {
            b += 1;
        }
        if b <= k {
            res = res.max(r - l + 1);
        } else {
            while r > 0 && l < r - 1 && s[l] == 'a' {
                l += 1;
            }
            if s[l] == 'b' {
                b -= 1;
            }
            l += 1;
        }
        r += 1;
    }
    let mut l = 0;
    let mut r = 0;
    let mut a = 0;
    while l < n && r < n {
        // println!("22 {} {}", l, r);
        if s[r] == 'a' {
            a += 1;
        }
        if a <= k {
            res = res.max(r - l + 1);   

        } else {
            while r > 0 && l < r - 1 && s[l] == 'b' {
                l += 1;
            }
            if s[l] == 'a' {
                a -= 1;
            }
            l += 1;
        }
        r += 1;
    }
    println!("{}", res);
}