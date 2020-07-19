use std::io;

fn main() {
    let mut buffer = String::new();
    let (a, b) = {
        io::stdin().read_line(&mut buffer).unwrap();
        let mut ws = buffer.split_whitespace();
        let a: i32 = ws.next().unwrap().parse().unwrap();
        let b: i32 = ws.next().unwrap().parse().unwrap();
        (a, b)
    };
    let result = if is_even(a, b) {
        "Even"
    } else {
        "Odd"
    };
    println!("{}", result);
}

fn is_even(a: i32, b: i32) -> bool {
    if (a * b) % 2 == 0 {
        return true;
    }
    return false;
}
