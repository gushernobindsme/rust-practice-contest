use std::io;

fn main() {
    // 1 行目
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n: i32 = buffer.trim().parse().unwrap();

    // 2 行目
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut ws = buffer.split_whitespace();

    let mut cards: Vec<i32> = vec![];
    for _i in 0..n {
        cards.push(ws.next().unwrap().parse().unwrap());
    }
    cards.sort_by(|a, b| b.cmp(a));

    let mut alice = 0;
    let mut bob = 0;

    for i in 0..cards.len() {
        if i % 2 == 0 {
            alice += cards.get(i).unwrap();
        } else {
            bob += cards.get(i).unwrap();
        }
    }

    println!("{}", alice - bob);
}
