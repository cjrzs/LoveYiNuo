fn input_nums() -> Vec<i32> {
    let mut nums = String::new();
    std::io::stdin().read_line(&mut nums).unwrap();
    nums.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let (n, m) = (nums[0], nums[1]);
        let (mut start, mut end, mut pre_time) = (m, m, 0);
        let mut flag = true;
        for _ in 0..n {
            let nums = input_nums();
            let (a, l, r) = (nums[0], nums[1], nums[2]);
            // println!("a: {}, l: {}, r: {}", a, l, r);
            if a - pre_time + end < l || start - a + pre_time > r{
                flag = false;
            }
            start = l.max(start - a + pre_time);
            end = r.min(end + a - pre_time);
            pre_time = a;
            // println!("pre_time: {}, end: {}", pre_time, end);
        }
        ans.push(if flag { "YES" } else { "NO" });
    }
    for x in ans {
        println!("{}", x);
    }
}

