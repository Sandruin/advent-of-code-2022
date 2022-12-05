use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    const STACK_AMOUNT: usize = 9;

    let (starting_stacks, procedure_text) = input.split_at(input.find("\n\n").unwrap() + 2);

    let mut stacks: [Vec<char>; STACK_AMOUNT] = Default::default();
    // could be made faster by not storing procedure and directly executing it, but this is logically cleaner
    let mut procedure: Vec<(usize, usize, usize)> = Default::default();

    starting_stacks.lines().rev().skip(2).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            // .inspect(|f| println!("{}", f))
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c));
    });

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

    procedure.iter().for_each(|p| {
        for _ in 0..(*p).0 {
            let c = stacks[(*p).1 - 1].pop().unwrap();
            stacks[(*p).2 - 1].push(c);
        }
    });

    let output: String = stacks.iter().map(|i| i.iter().last().unwrap()).collect();

    println!("{:?}", output);
}
