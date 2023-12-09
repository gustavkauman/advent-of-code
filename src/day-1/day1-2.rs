const DIGITS_AS_STRS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn main() {
    let res = include_bytes!("./day1.input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            if line.len() != 0 {
                (0..line.len()).find_map(|i| str_as_digit(line, i)).unwrap() * 10
                    + (0..line.len())
                        .rev()
                        .find_map(|i| str_as_digit(line, i))
                        .unwrap()
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("Result is {res}");
}

fn str_as_digit(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(DIGITS_AS_STRS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}
