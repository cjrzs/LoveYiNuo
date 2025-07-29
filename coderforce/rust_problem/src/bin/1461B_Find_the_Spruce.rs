fn input_str() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}

fn main() {
    let t = input_str().parse().unwrap();
    let mut res: Vec<i32> = Vec::new();
    for _ in 0..t {
        let nums: Vec<usize> = input_str().split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
        let (n, m) = (nums[0], nums[1]);
        let mut arr: Vec<Vec<char>> = Vec::new();
        for _ in 0..n {
            let line: Vec<char> = input_str().chars().collect();
            arr.push(line);
        }
        let mut f = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 1..m + 1 {
                if arr[i][j - 1] == '*' {
                    f[i][j] = f[i][j - 1] + 1;
                } else {
                    f[i][j] = f[i][j - 1];
                }
            }
        }
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if arr[i][j] != '*' {
                    continue;
                }
                ans += 1;
                for k in 1..n - i {
                    if j + k >= m || j < k {
                        break;
                    }
                    // println!("{} {} {} {} {} {}", i, j, k, f[i + k][j + k + 1], f[i + k][j - k], k * 2 + 1);
                    if f[i + k][j + k + 1] - f[i + k][j - k] == k * 2 + 1 {
                        ans += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        res.push(ans);
    }
    for x in res {
        println!("{}", x);
    }
}


