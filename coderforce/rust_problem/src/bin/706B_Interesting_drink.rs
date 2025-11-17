use std::{io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x: &str| x.parse().unwrap()).collect()
}

fn binary_search(nums: &Vec<i64>, target: i64) -> usize{
    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
        let mid: usize = (l + r + 1) / 2;
        // println!("{} {} {} {}", mid, nums[mid], l, r);
        if nums[mid] > target {
            r = mid - 1;
        } else {
            l = mid;
        }
    }
    if r == 0 {
        if nums[0] <= target {
            return 1;
        } else {
            return 0;
        }
    } else {
        r + 1
    }
}

fn main() {
    let _n = input_nums()[0];
    let stores = input_nums();
    let mut stores_sort = stores.clone();
    stores_sort.sort();
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let q = input_nums()[0];
        ans.push(binary_search(&stores_sort, q));
        // println!("---");
    }
    for x in ans {
        println!("{}", x);
    }
}

