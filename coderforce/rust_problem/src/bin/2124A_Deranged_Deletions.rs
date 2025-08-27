fn input_num() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_num();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_num();
        let n = nums[0] as usize;
        let nums = input_num();
        if n == 1 {
            ans.push((-1, -1));
            continue;
        }
        let mut cur_max = 0;
        let mut flag = false;
        for i in 0..n {
            if nums[i] < cur_max {
                ans.push((cur_max, nums[i]));
                flag = true;
                break;
            }
            cur_max = nums[i];
        }
        if !flag {
            ans.push((-1, -1));
        }
    }
    for i in 0..ans.len() {
        let (a, b) = ans[i];
        if a == -1 && b == -1 {
            println!("NO");
        } else {
            println!("YES");
            println!("2");
            println!("{} {}", a, b);
        }
    }
}

