fn main() {
    let mut s = include_str!("../../inputs/day1_input.txt");

    let lines = s.lines().map(|line| {

        let digit_chars = line
            .chars()
            .filter(|c| {
                c.is_digit(10)
            }).collect::<Vec<_>>();

        vec![*digit_chars.first().unwrap(), *digit_chars.last().unwrap()].iter().collect::<String>().parse::<u32>().unwrap()
    }).sum::<u32>();

    println!("{}", lines)
}
