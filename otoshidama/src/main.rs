use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut ws = buffer.split_whitespace();
    let n: i32 = ws.next().unwrap().parse().unwrap(); // お札の枚数
    let y: i32 = ws.next().unwrap().parse().unwrap(); // 合計金額

    let mut success = false;
    'outer: for i in 0..=n {
        for j in 0..=n - i {
            let yukichi = i; // 一万円
            let higuchi = j; // 五千円
            let noguchi = n - yukichi - higuchi; // 千円
            if yukichi + higuchi + noguchi == n {
                let amount = yukichi * 10000 + higuchi * 5000 + noguchi * 1000;
                if amount == y {
                    println!("{} {} {}", yukichi, higuchi, noguchi);
                    success = true;
                    break 'outer;
                }
            }
        }
    }

    if !success {
        println!("-1 -1 -1");
    }
}
