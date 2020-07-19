use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut chars = buffer.chars();
    let s1 = chars.next().unwrap();
    let s2 = chars.next().unwrap();
    let s3 = chars.next().unwrap();
    let i1: i8 = if s1 == '1' { 1 } else { 0 };
    let i2: i8 = if s2 == '1' { 1 } else { 0 };
    let i3: i8 = if s3 == '1' { 1 } else { 0 };
    println!("{}", i1 + i2 + i3);
}
