fn input_nums() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let t = input_nums().parse().unwrap();
    let mut res = Vec::new();
    for _ in 0..t {
        let n = input_nums().parse::<i64>().unwrap();
        let s1s = input_nums().chars().collect::<Vec<char>>();
        let s2s = input_nums().chars().collect::<Vec<char>>();
        let mut s1 = vec![0u8; n as usize + 2];
        let mut s2 = vec![0u8; n as usize + 2];
        for i in 0..n {
            s1[i as usize + 1] = s1s[i as usize].to_digit(10).unwrap() as u8;
            s2[i as usize + 1] = s2s[i as usize].to_digit(10).unwrap() as u8;
        }
        let mut ans = String::with_capacity(n as usize + 1);
        let (mut l, mut r) = (1, n);
        for i in 1..=n + 1 {
            let v1 = if l <= i.saturating_sub(1) {s2[i as usize - 1]} else {2};
            let v2 = if i <= n && i <= r {s1[i as usize]} else {2};
            let c = v1.min(v2);
            ans.push(if c == 0 {'0'} else {'1'});
            let ok1 = c == v1;
            let ok2 = c == v2;
            if ok1 && !ok2 {
                r = r.min(i - 1);
            } else if !ok1 && ok2 {
                l = l.max(i);
            }
        }
        let cnt = if r >= l {r - l + 1} else {0};
        res.push((cnt, ans));
    }
    for (ans, s) in res {
        println!("{}", s);
        println!("{}", ans);
    }
}
