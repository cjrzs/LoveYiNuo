fn input_nums() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0] as usize;
        let nums = input_nums();
        let mut cur_min = n as i32 + 1;
        let mut cur_ans = n as i32 + 1;
        let mut res = 0;
        for i in 0..n {
            if nums[i] > cur_min && nums[i] < cur_ans {
                res += 1;
                cur_ans = nums[i];
            }
            cur_min = cur_min.min(nums[i]);
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}

