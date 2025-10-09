use std::collections::VecDeque;

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let n = input_nums()[0];
    let nums = input_nums();
    let mut ans = 1;
    let mut l = 0;
    let mut mi_deque = VecDeque::new();
    let mut ma_deque = VecDeque::new();
    for i in 0..n {
        while !mi_deque.is_empty() && *mi_deque.back().unwrap() > nums[i as usize] {
            mi_deque.pop_back();
        }
        mi_deque.push_back(nums[i as usize]);
        while !ma_deque.is_empty() && *ma_deque.back().unwrap() < nums[i as usize] {
            ma_deque.pop_back();
        }
        ma_deque.push_back(nums[i as usize]);
        while ma_deque.front().unwrap() - mi_deque.front().unwrap() > 1 {
            let a = nums[l as usize];
            if a == *mi_deque.front().unwrap() {
                mi_deque.pop_front();
            }
            if a == *ma_deque.front().unwrap() {
                ma_deque.pop_front();
            }
            l += 1;
        }
        ans = ans.max(i - l + 1);
        // println!("{:?}, {:?}", mi_deque, ma_deque);
        // println!("{} {}", l, i);
    }
    println!("{}", ans);
}




