use std::collections::HashMap;

fn input_nums() -> Vec<i64> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut res = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0] as usize;
        let nums = input_nums();
        let ans = solve(nums, n);
        res.push(ans);
    }
    for x in res {
        println!("{}", x);
    }
}

fn solve(nums: Vec<i64>, n: usize) -> i64 {
    let tmp: i64 = nums.iter().sum();
    // println!("sun: {}", tmp);
    let k = 2f64 * tmp as f64 / n as f64;
    if (2 * tmp) % n as i64 != 0 {
        return 0;
    }
    // println!("avg: {}", k);
    let mut hash_map = HashMap::new();
    for &x in &nums {
        *hash_map.entry(x).or_insert(0) += 1;
    }
    let mut res = 0i64;
    for x in nums {
        let y = (k - x as f64) as i64;
        if let Some(num) = hash_map.get(&y)  {
            res += num;
            if x == y {
                res -= 1;
            }
        }
    }
    res / 2
}