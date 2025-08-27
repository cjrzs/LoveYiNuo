fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let n = input().trim().parse::<i64>().unwrap();
    let line = input();
    let s: Vec<&str> = line.trim().split_whitespace().collect::<Vec<&str>>();
    // println!("{:?}", s);
    let mut cnt = vec![vec![0; 46]; 6];
    for i in 0..n {
        let num = s[i as usize];
        let tmp = num.chars().map(|x| x.to_digit(10).unwrap() as u64).sum::<u64>();
        cnt[num.len()][tmp as usize] += 1;
    }
    // for x in cnt.iter() {
    //     println!("{:?}", x);
    // }
    let mut ans: i64 = 0;
    // for i in 0..n {
    //     let num = s[i as usize];
    //     let len = num.len();
    //     for j in 1..=5 {
    //         if (len + j) % 2 == 1 {
    //             continue;
    //         }
    //         let mid = (len + j) / 2;
    //         // 左半部分数字和
    //         let left: u32 = num[..mid]
    //             .chars()
    //             .map(|x| x.to_digit(10).unwrap())
    //             .sum();
    //         // 右半部分数字和
    //         let right: u32 = 
    //             num[mid..]
    //                 .chars()
    //                 .map(|x| x.to_digit(10).unwrap())
    //                 .sum();
    //         let tmp = left as i64 - right as i64;
    //         if tmp > 0 {
    //             ans += hash_map[j][tmp as usize];
    //         }
    //         println!("{} {} {} {}", left, right, j, tmp);
    //         // 左半部分数字和
    //         let right: u32 = num[len - mid..]
    //             .chars()
    //             .map(|x| x.to_digit(10).unwrap())
    //             .sum();
    //         // 右半部分数字和
    //         let left: u32 = num[..len - mid]
    //             .chars()
    //             .map(|x| x.to_digit(10).unwrap())
    //             .sum();
    //         let tmp2 = right as i64 - left as i64;
    //         println!("{} {} {} {}", left, right, j, tmp2);
    //         if tmp2 > 0 && tmp2 != tmp{
    //             ans += hash_map[j][tmp2 as usize];
    //         }
    //     }
    // }
    
    // 第一趟：当作左串 L，只枚举 len_r ≤ len(L)，并且 (len(L)+len_r)%2==0
    for L in &s {
        let len_l = L.len();
        // 先算前缀和
        let digs: Vec<usize> = L.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        let mut pre = vec![0usize; len_l + 1];
        for i in 0..len_l {
            pre[i + 1] = pre[i] + digs[i];
        }
        let parity = len_l % 2;
        // len_r 从 parity, parity+2, ... ≤ len_l
        for len_r in (parity..=len_l).step_by(2) {
            let total_len = len_l + len_r;
            let h = total_len / 2; // mid
            // mid ≤ len_l 保证 h ≤ len_l
            let suml = pre[h];
            let sumr = pre[len_l] - pre[h];
            let diff = suml as i64 - sumr as i64;
            if diff >= 0 && (diff as usize) <= 45 {
                ans += cnt[len_r][diff as usize];
            }
        }
    }
    
    // 第二趟：当作右串 R，只枚举 len_l < len(R)，并且 (len(R)+len_l)%2==0
    for R in &s {
        let len_r = R.len();
        let digs: Vec<usize> = R.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        let mut pre = vec![0usize; len_r + 1];
        for i in 0..len_r {
            pre[i + 1] = pre[i] + digs[i];
        }
        let parity = len_r % 2;
        // len_l 从 parity, parity+2, ... < len_r
        for len_l in (parity..len_r).step_by(2) {
            let total_len = len_r + len_l;
            let h = total_len / 2; // mid
            // 这里 h ≤ len_r，因为 len_l < len_r
            // suml 取后缀长度 h，sumr 取前缀长度 len_r-h
            let suml = pre[len_r] - pre[len_r - h];
            let sumr = pre[len_r - h];
            let diff = suml as i64 - sumr as i64;
            if diff >= 0 && (diff as usize) <= 45 {
                ans += cnt[len_l][diff as usize];
            }
        }
    }
    println!("{}", ans);
}

