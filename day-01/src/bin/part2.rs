const DIGIT_MAP: &[(u32, &str)] = &[
    (1, "1"),
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5"),
    (6, "6"),
    (7, "7"),
    (8, "8"),
    (9, "9"),
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> Option<u32> {
    input
        .split_terminator(char::is_whitespace)
        .map(|line| {
            let mut index = (i32::MAX, i32::MIN);
            let (mut first, mut last) = (0, 0);
            for (val, str) in DIGIT_MAP {
                let matches = line.match_indices(str).collect::<Vec<_>>();
                if let Some((i, _)) = matches.first() {
                    let i = *i as i32;
                    if i < index.0 {
                        index.0 = i;
                        first = *val;
                    }
                }

                if let Some((i, _)) = matches.last() {
                    let i = *i as i32;
                    if i >= index.1 {
                        index.1 = i;
                        last = *val;
                    }
                }
            }
            Some(first * 10 + last)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../input-example2.txt");
        let result = part2(input);
        assert_eq!(Some(281), result);
    }
}
