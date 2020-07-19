use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let a: i32 = buffer.trim().parse().unwrap();

    let mut buffer = String::new();
    let (b, c) = {
        io::stdin().read_line(&mut buffer).unwrap();
        let mut ws = buffer.split_whitespace();
        let b: i32 = ws.next().unwrap().parse().unwrap();
        let c: i32 = ws.next().unwrap().parse().unwrap();
        (b, c)
    };

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let s: String = buffer.trim_end().to_owned();

    println!("{} {}", a + b + c, s);
}
