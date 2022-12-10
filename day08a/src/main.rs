fn main() {
    let input = include_str!("../test.txt");

    // 3. unique indices
    // 4. sum over values

    let row_count: usize = input.lines().count();
    let column_count: usize = input.lines().next().unwrap().chars().count();

    // 1. get matrix of values
    let mut forest: Vec<Vec<u32>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // 2. get max index per side
    // rows left to right
    let mut indices: Vec<(usize, usize)> = Default::default();
    indices.extend(forest.iter().enumerate().map(|(row_index, i)| {
        // get column_index
        (
            row_index,
            i.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(iter, _)| iter)
                .unwrap(),
        )
    }));

    let mut trees: u32 = 0;

    let output = 0;
    println!("{:?}", indices);
}
