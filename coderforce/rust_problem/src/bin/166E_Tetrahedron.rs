use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    stdin.lock().read_line(&mut line).unwrap();
    let n: i64 = line.trim().parse().unwrap();
    let mut ai: i64 = 1; // ai D到D 经过i步的方案数量。
    let mut bi: i64 = 0; // bi D到非D 经过i步的方案数量。
    let r#mod: i64 = 1000000007;
    for _ in 1..n + 1 {
        let a: i64 = bi % r#mod;
        let b: i64 = (3 * ai % r#mod + 2 * bi % r#mod) % r#mod;
        ai = a;
        bi = b;
    }
    println!("{}", ai);
}


