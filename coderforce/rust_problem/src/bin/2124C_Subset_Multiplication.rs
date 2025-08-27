fn input_nums() -> Vec<i64> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let _n = nums[0] as usize;
        let nums = input_nums();
        let mut lcm_val = 1;
        let mut gcd_val = 0;
        for &x in nums.iter().rev() {
            gcd_val = gcd(gcd_val, x);
            lcm_val = lcm(lcm_val, x / gcd(x, gcd_val));
        }
        ans.push(lcm_val);
    }
    for x in ans {
        println!("{}", x);
    }
}
