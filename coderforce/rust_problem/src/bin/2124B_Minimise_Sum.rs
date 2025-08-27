fn input_nums() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let _n = nums[0] as usize;
        let nums = input_nums();
        ans.push((nums[0] + nums[1]).min(nums[0] * 2));
    }
    for x in ans {
        println!("{}", x);
    }
}