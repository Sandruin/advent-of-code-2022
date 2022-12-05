use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    const STACK_AMOUNT: usize = 9;

    let (starting_stacks, procedure_text) = input.split_at(input.find("\n\n").unwrap() + 2);

    let mut stacks: [Vec<char>; STACK_AMOUNT] = Default::default();
    // could be made faster by not storing procedure and directly executing it, but this is logically cleaner
    let mut procedure: Vec<(usize, usize, usize)> = Default::default();

    // capture starting position in stacks as vectors for each crate stack
    starting_stacks.lines().rev().skip(2).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            // .inspect(|f| println!("{}", f))
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c));
    });

    // capture move order in procedure as tuple (how many, from, to)
    procedure_text.lines().for_each(|line| {
        procedure.push(
            line.split(' ')
                .skip(1)
                .step_by(2)
                .map(|s| s.parse::<usize>().unwrap())
                // .inspect(|f| println!("{}", f))
                .collect_tuple()
                .unwrap(),
        );
    });

    // go through procedure vector and switch crates
    // this time split off multiple crates at once and append them again
    procedure.iter().for_each(|p| {
        let moved_crates = stacks[(*p).1 - 1].split_off(stacks[(*p).1 - 1].len() - (*p).0);
        moved_crates.iter().for_each(|c| {
            stacks[(*p).2 - 1].push(*c);
        })
    });

    // get top-most elements of all stacks and combine them to a string
    let output: String = stacks.iter().map(|i| i.iter().last().unwrap()).collect();

    println!("{:?}", output);
}
