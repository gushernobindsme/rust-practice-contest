use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let a: i32 = buffer.trim().parse().unwrap(); // 500 円玉

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let b: i32 = buffer.trim().parse().unwrap(); // 100 円玉

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let c: i32 = buffer.trim().parse().unwrap(); // 50 円玉

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let x: i32 = buffer.trim().parse().unwrap(); // 合計金額

    let mut count = 0;
    for i_a in 0..a + 1 {
        for i_b in 0..b + 1 {
            for i_c in 0..c + 1 {
                let total = 500 * i_a + 100 * i_b + 50 * i_c;
                if total == x {
                    count += 1
                }
            }
        }
    }
    println!("{}", count);
}
