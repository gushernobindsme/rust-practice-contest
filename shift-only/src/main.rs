use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let _count: u32 = buffer.trim().parse().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let ws = buffer.split_whitespace();
    let mut numbers: Vec<u32> = vec![];
    for w in ws {
        numbers.push(w.parse().unwrap());
    }

    let mut execution_count = 0;
    while is_executable(&numbers) {
        numbers = numbers.into_iter().map(|n| n / 2).collect();
        execution_count += 1;
    }
    println!("{}", execution_count);
}

fn is_executable(numbers: &Vec<u32>) -> bool {
    let mut result = true;
    for number in numbers {
        if (number % 2) != 0 {
            result = false;
            break;
        }
    }
    result
}
