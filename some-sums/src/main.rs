use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut ws = buffer.split_whitespace();
    let n: i32 = ws.next().unwrap().parse().unwrap();
    let a: i32 = ws.next().unwrap().parse().unwrap();
    let b: i32 = ws.next().unwrap().parse().unwrap();

    let mut total = 0;
    for i in 0..n + 1 {
        let sum = sum(i);
        if a <= sum && sum <= b {
            total += i;
        }
    }
    println!("{}", total);
}

fn sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
