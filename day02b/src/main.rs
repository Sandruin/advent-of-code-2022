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
        // outcome = 3 * b
        // pick = (a + b) + 3 clamped between [1:3]
        .map(|(a, b)| 3 * b + 1 + (a + b + 2) % 3)
        .sum::<i16>();

    println!("{}", output);
}
