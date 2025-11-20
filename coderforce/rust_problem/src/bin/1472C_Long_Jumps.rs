use std::{collections::HashMap, i64, io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn func(i: i64, nums: &Vec<i64>, d: &mut HashMap<usize, i64>, n: usize) -> i64 {
    let (mut res, mut a) = (nums[i as usize], nums[i as usize]);
    while (a + i) <= n as i64 {
        a = a + i;
        let a_usize = a as usize;
        if d.contains_key(&a_usize) {
            let k = d.get(&a_usize).unwrap();
            res += k;
            d.insert(i as usize, res);
            return res;
        }
        res += nums[a as usize];
    }
    d.insert(i as usize, res);
    res
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0] as usize;
        let mut nums = input_nums();
        nums.insert(0, 0);
        let mut res = 0;
        let mut d = HashMap::new();
        for i in (1..n + 1).rev() {
            res = res.max(func(i as i64, &nums, &mut d, n));
        }
        ans.push(res);
        // println!("{:?}", d);
    }
    for x in ans {
        println!("{}", x);
    }
}


