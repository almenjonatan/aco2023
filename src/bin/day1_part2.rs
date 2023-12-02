fn number(n: &[u8]) -> Option<&str> {
    match n {
        [b'o', b'n', b'e', ..] | [b'1', ..] => Some("1"),
        [b't', b'w', b'o', ..] | [b'2', ..] => Some("2"),
        [b't', b'h', b'r', b'e', b'e', ..] | [b'3', ..] => Some("3"),
        [b'f', b'o', b'u', b'r', ..] | [b'4', ..] => Some("4"),
        [b'f', b'i', b'v', b'e', ..] | [b'5', ..] => Some("5"),
        [b's', b'i', b'x', ..] | [b'6', ..] => Some("6"),
        [b's', b'e', b'v', b'e', b'n', ..] | [b'7', ..] => Some("7"),
        [b'e', b'i', b'g', b'h', b't', ..] | [b'8', ..] => Some("8"),
        [b'n', b'i', b'n', b'e', ..] | [b'9', ..] => Some("9"),
        _ => None
    }
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(|l| {
        let bytes = l.as_bytes();

        let numbers = (0..bytes.len())
            .filter_map(|i| number(&bytes[i..]))
            .collect::<Vec<_>>();

        [numbers.first().unwrap(), numbers.last().unwrap()].join("").parse::<u32>().unwrap()
    }).sum()
}

fn main() {
    let input = include_str!("../../inputs/day1_input.txt");
    println!("{}", part2(input))
}
