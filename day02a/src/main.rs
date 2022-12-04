fn main() {
    let input = include_str!("../input.txt");

    let output = input
        .lines()
        .map(|game| {
            (
                game.chars().nth(0).unwrap() as i16 - b'A' as i16,
                game.chars().nth(2).unwrap() as i16 - b'X' as i16,
            )
        })
        // rem_euclid for remainder (always positive) operation and not modulo
        .map(|(a, b)| b + 1 + 3 * ((1 + b - a).rem_euclid(3)))
        .sum::<i16>();

    println!("{}", output);
}
