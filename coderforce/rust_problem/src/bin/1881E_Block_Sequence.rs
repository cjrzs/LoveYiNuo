fn input_nums() -> Vec<usize> {
    let mut nums = String::new();
    std::io::stdin().read_line(&mut nums).unwrap();
    nums.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn get(f: &Vec<usize>, i: usize, n: usize) -> usize {
    if i > n {
        return n + 1;
    }
    if i == n {
        return 0;
    }
    return f[i];
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0];
        let nums = input_nums();
        let mut f = vec![n + 1; n];
        f[n - 1] = 1;
        for i in (0..n - 1).rev() {
            f[i] = get(&f, i + nums[i] + 1, n).min(f[i + 1] + 1);
        }
        // println!("{:?}", f);
        ans.push(f[0]);
    }
    for x in ans {
        println!("{}", x);
    }
}
