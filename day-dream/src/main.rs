use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let s: &str = buffer.trim();

    let words = ["dream", "dreamer", "erase", "eraser"];
    let try_drop_tail = |s: &str| -> Option<usize> {
        words
            .into_iter()
            .filter_map(|word| {
                if s.ends_with(word) {
                    Some(s.len() - word.len())
                } else {
                    None
                }
            })
            .next()
    };

    let mut sub_str = s;

    while sub_str.len() > 0 {
        match try_drop_tail(sub_str) {
            Some(val) => sub_str = &s[0..val],
            None => {
                println!("NO");
                return;
            }
        }
    }

    println!("YES");
}
