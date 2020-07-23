use std::io;

fn main() {
    // 1 行目
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n: i32 = buffer.trim().parse().unwrap();

    // 2 行目以降
    let mut records: Vec<(i32, i32, i32)> = Vec::new();
    for _i in 0..n {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut ws = buffer.split_whitespace();
        let t: i32 = ws.next().unwrap().parse().unwrap();
        let x: i32 = ws.next().unwrap().parse().unwrap();
        let y: i32 = ws.next().unwrap().parse().unwrap();
        records.push((t, x, y));
    }

    records.insert(0, (0,0,0));

    let success = records[..].windows(2).all(|w| {
        let (t, x, y) = w[0];
        let (nt, nx, ny) = w[1];
        let time = nt- t;
        let dist = (nx - x).abs() + (ny - y).abs();
        dist <= time && time % 2 == dist % 2
    });

    println!("{}", if success { "Yes" } else { "No" });
}
