fn input_nums() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let n = input_nums()[0] as usize;
    let nums = input_nums();
    let mut pre = vec![0; n + 1];
    let mut q = Vec::new();
    for x in nums {
        if x > 0 {
            q.push(0);
        } else {
            q.push(1);
        }
    }
    for i in 1..=n {
        pre[i] = pre[i - 1] ^ q[i - 1];
    }
    // println!("{:?}", pre);
    let mut cnt_even = 1;
    let mut cnt_odd = 0;
    for i in 1..=n{
        if pre[i] == 0 {
            cnt_even += 1;
        } else {
            cnt_odd += 1;
        }
    }
    let even = cnt_even * (cnt_even - 1) / 2 + cnt_odd * (cnt_odd - 1) / 2;
    println!("{} {}", n * (n + 1) / 2 - even,  even);
}

