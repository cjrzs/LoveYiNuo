use std::cmp;
use std::io::{self, BufRead};
// use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = nums[0];
    // let mut res: i32 = 0;
    // let mut cache: HashMap<i32, i32> = HashMap::new();
    let mut f: Vec<i32> = vec![-1; 120000];
    f[0] = 0;
    for i in 1..=3 {
        for j in nums[i]..=n {
            if f[j as usize - nums[i] as usize] < 0 {
                continue;
            }
            f[j as usize] = cmp::max(f[j as usize], f[j as usize - nums[i] as usize] + 1);
        }
    }
    // dfs(n, a, b, c, 0, &mut res, &mut cache);
    println!("{}", f[n as usize]);
}

// fn dfs(u: i32, a: i32, b: i32, c: i32, s: i32, res: &mut i32, cache: &mut HashMap<i32, i32>) {
//     if u <= 0 {
//         if u == 0 {
//             *res = (*res).max(s);
//         }
//         return 
//     }
//     // 检查缓存
//     if let Some(&cached_s) = cache.get(&u) {
//         *res = (*res).max(s + cached_s);
//         return;
//     }// 记录当前结果
//     let old_res = *res;
//     dfs(u - a, a, b, c, s + 1, res, cache);
//     dfs(u - b, a, b, c, s + 1, res, cache);
//     dfs(u - c, a, b, c, s + 1, res, cache);
//     if *res > old_res {
//         cache.insert(u, *res - s);
//     }
// }
