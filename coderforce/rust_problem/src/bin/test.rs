use std::{collections::HashSet, io};

fn input_nums() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_hash(s: &[u8], left: usize, right: usize) -> u64 {
    let mut hash = 0;
    const MOD: u64 = 1_000_000_007;
    for i in left..right {
        hash = (hash * 131 + s[i] as u64) % MOD;
    }
    hash
}

fn check(s1: &[u8], s2: &[u8], len: usize) -> Option<usize> {
    if len == 0 || s1.len() < len || s2.len() < len {
        return Some(0);
    }
    let mut hash_set = HashSet::new();
    for i in 0..s1.len() - len {
        hash_set.insert(get_hash(s1, i, i + len));
    }
    for i in 0..s2.len() - len {
        let t = get_hash(s2, i, i + len);
        if hash_set.contains(&t) {
            return Some(i);
        }
    }
    None
}
// O(n)

fn main() {
    let s1 = input_nums();
    let s2 = input_nums();
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let mut max_len = 0;
    let mut start = 0;
    let (mut l, mut r) = (0, s1.len().min(s2.len()));
    // O(log n)
    while l <= r {
        let mid = l + (r - l) / 2;
        if let Some(a) = check(b1, b2, mid) {
            l = l + 1;
            max_len = mid;
            start = a;
        } else {
            r = r - 1;
        }
    }
    println!("{} {}", start, max_len);
    println!("{:?}", String::from_utf8_lossy(&b2[start..start + max_len]).to_string());
}

// youhaveablueball
// igetuhaveablumauhaveablueydd

// O(n log n)

