use std::{collections::HashMap, io};

fn main() {
    let mut lines = io::stdin().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut d = HashMap::new();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut it = line.split_whitespace();
        let val: usize = it.next().unwrap().parse().unwrap();
        let nums: Vec<char> = it.next().unwrap().trim().chars().collect();
        let mut res = 0;
        for x in nums {
            if x == 'A' {
                res += 1;
            } else if x == 'B' {
                res += 2;
            } else if x == 'C' {
                res += 4;
            }
        }
        d.entry(res)
            .and_modify(|x| {
                if *x > val {
                    *x = val;
                }
            })
            .or_insert(val);
        
    }
    let mut ans = i64::MAX;
    if let Some(x) = d.get(&7) {
        ans = ans.min(*x as i64);
    }
    for (&i, &x) in &d {
        for (&j, &y) in &d {
            if i | j == 7 {
                ans = ans.min(x as i64 + y as i64);
            }
        }
    }
    for (&i, &x) in &d {
        for (&j, &y) in &d {
            for (&k, &q) in &d {
                if i | j | k == 7 {
                    ans = ans.min(x as i64 + y as i64 + q as i64);
                }
            }
        }
    }
    println!("{}", if ans != i64::MAX { ans } else {-1});
}


