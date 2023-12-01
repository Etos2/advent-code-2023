const RADIX: u32 = 10;

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> Option<u32> {
    input.split_terminator(char::is_whitespace).map(|line| {
        let first = line.chars().find(char::is_ascii_digit).map(|c| char::to_digit(c, RADIX))??;
        let last = line.chars().rfind(char::is_ascii_digit).map(|c| char::to_digit(c, RADIX))??;
        Some(first * 10 + last)
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input-example1.txt");
        let result = part1(input);
        assert_eq!(Some(142), result);
    }
}