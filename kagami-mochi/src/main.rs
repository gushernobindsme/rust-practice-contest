use std::collections::HashSet;
use std::io;

fn main() {
    // 1 行目
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n: i32 = buffer.trim().parse().unwrap();

    // 2 行目以降
    let mut d: Vec<i32> = Vec::new();
    for _i in 0..n {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        d.push(buffer.trim().parse().unwrap());
    }

    let unique_d: HashSet<i32> = d.into_iter().collect();

    println!("{}", unique_d.len());
}
