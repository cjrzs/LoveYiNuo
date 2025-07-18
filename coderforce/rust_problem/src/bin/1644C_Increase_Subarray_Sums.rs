use std::io::BufRead;

fn input_nums() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let (n, x) = (nums[0] as usize, nums[1]);
        let nums = input_nums();
        // let mut res = 0;
        let mut mx = vec![i32::MIN; n + 1];
        mx[0] = 0;
        for l in 0..n {
            let mut s = 0;
            for r in l..n {
                s += nums[r as usize];
                mx[r - l + 1] = mx[r - l + 1].max(s);
            }
        }
        // println!("{:?}", mx);
        let mut tmp = Vec::new();
        for k in 0..n + 1 {
            let mut bts = 0;
            for l in 0..n + 1 {
                bts = bts.max(mx[l] + (l as i32).min(k as i32) * x);
                // println!("{} {} {}", l, k, bts);
            }
            tmp.push(bts);
        }
        ans.push(tmp.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" "));
    }
    for x in ans {
        println!("{}" ,x);
    }
}


