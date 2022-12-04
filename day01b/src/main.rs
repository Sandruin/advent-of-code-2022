fn main() {
    let input = include_str!("../input.txt");

    let mut cals = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    cals.sort();
    let output: u32 = cals.iter().rev().take(3).sum();

    println!("{}", output);
}
