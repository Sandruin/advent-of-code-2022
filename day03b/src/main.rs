fn main() {
    let input = include_str!("../input.txt");

    // split at newline
    let output = input
        // split into lines
        .lines()
        // collect to &str vector to allow chunking
        .collect::<Vec<&str>>()
        // put 3 lines together
        .chunks(3)
        .map(|chunks| {
            chunks[0]
                // iterate over characters of first line
                .chars()
                // only yield chars that are contained in second and third line
                .filter(|c| chunks[1].contains(*c) && chunks[2].contains(*c))
                .map(|c1| {
                    if c1 >= 'a' {
                        c1 as i16 - 'a' as i16 + 1
                    } else {
                        c1 as i16 - 'A' as i16 + 27
                    }
                })
                // should only be one value anyway
                .next()
                .unwrap_or_default()
        })
        .sum::<i16>();

    println!("{}", output);
}
