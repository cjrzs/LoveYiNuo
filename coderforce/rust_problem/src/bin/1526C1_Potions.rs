use std::{cmp::Reverse, collections::BinaryHeap, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let _: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let nums: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // f[i][k] 从前i个药水中选择，k表示喝的药水总数。
    // 如果不喝药水i 则f[i] = f[i - 1][k];
    // 如果喝药水i 必须有f[i - 1][k - 1] + nums[i] > 0
    // f[i][k] = max(f[i - 1][k - 1] + nums[i], f[i - 1][k])
    // let mut f = vec![vec![i64::MIN; n + 1]; n + 1];
    // f[0][0] = 0;
    // for i in 1..n + 1 {
    //     for j in 0..i + 1 {
    //         f[i][j] = f[i - 1][j];
    //         if j > 0 && f[i - 1][j - 1] != i64::MIN && f[i - 1][j - 1] + nums[i - 1] >= 0 {
    //             f[i][j] = f[i][j].max(f[i - 1][j - 1] + nums[i - 1])
    //         }
    //         // println!("{}", f[i][j]);
    //     }
    // }
    // let mut res = 0;
    // for i in 0..n + 1 {
    //     // println!("{} {}", i, f[n][i]);
    //     if f[n][i] >= 0 {
    //         res = i
    //     }
    // }
    let mut health = 0;
    let mut heap = BinaryHeap::new();
    for x in nums {
        health += x;
        heap.push(Reverse(x));
        if health < 0 {
            if let Some(Reverse(y)) = heap.pop() {
                health -= y;
            }
        }
    }
    println!("{}", heap.len());
}


