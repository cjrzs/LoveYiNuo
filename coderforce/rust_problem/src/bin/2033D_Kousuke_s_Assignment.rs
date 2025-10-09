use std::collections::HashSet;

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut res = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0];
        let nums = input_nums();
        let mut ans = 0;
        let mut pre = vec![0; n as usize + 1];
        for i in 1..n + 1 {
            pre[i as usize] = pre[i as usize - 1] + nums[i as usize - 1];
        }
        let mut hash_set = HashSet::new();
        hash_set.insert(0);
        let mut i = 1;
        while i < n + 1 {
            let x = pre[i as usize];
            if hash_set.contains(&x) {
                ans += 1;
                hash_set.clear();
                hash_set.insert(x);
            } else {
                hash_set.insert(x);
            }
            i += 1;
        }
        res.push(ans);
    }
    for x in res {
        println!("{}", x);
    }
}
