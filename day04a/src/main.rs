fn main() {
    let input = include_str!("../input.txt");

    let output = input
        .lines()
        .map(|line| {
            // transform input to tuple of range interval
            let (l, r) = line.split_once(',').unwrap();
            let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        // check range bounds
        .filter(|(a, b, c, d)| (a <= c && b >= d) || (c <= a && d >= b))
        .count();

    println!("{}", output);
}
