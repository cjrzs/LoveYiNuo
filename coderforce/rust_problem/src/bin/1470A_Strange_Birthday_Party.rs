use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let [_n, m] = [nums[0], nums[1]];
        let friends = input_nums();
        let mut sort_friends: Vec<(usize, &i64)> = friends.iter().enumerate().collect();
        let costs = input_nums();
        sort_friends.sort_by(|a, b| {
            let va = costs[*(a.1) as usize - 1];
            let vb = costs[*(b.1) as usize - 1];
            vb.cmp(&va)
        });
        let mut sort_cost: Vec<(usize, &i64)> = costs.iter().enumerate().collect();
        sort_cost.sort_by_key(|&(_, y)| y);
        // println!("{:?}", sort_friends);
        // println!("{:?}", sort_cost);
        let mut res: i64 = 0;
        let mut i: usize = 0; // 当前使用的礼品
        let mut present = 0;
        for (u, _) in sort_friends {
            if i < m as usize {
                present = *(sort_cost[i].1);
            }
            let k = costs[friends[u] as usize - 1];
            // println!("x: {} present: {} k: {} ", x, present, k);
            if k <= present {
                res += k;
            } else {
                if i < m as usize {
                    res += present;
                    i += 1
                }
            }
        }
        ans.push(res);
    }
    for x in ans {
        println!("{x}");
    }
}
