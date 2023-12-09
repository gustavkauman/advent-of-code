fn main() {
    let res = include_bytes!("./day1.input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            if line.len() != 0 {
                ((line.iter().find(|b| b.is_ascii_digit()).unwrap() - b'0') * 10
                    + line.iter().rev().find(|b| b.is_ascii_digit()).unwrap()
                    - b'0') as usize
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("Result is {res}");
}
