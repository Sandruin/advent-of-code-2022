use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let mut last_four_letters: [char; 14] = Default::default();

    let mut index = 0;

    for (i, char) in input.chars().enumerate() {
        // push new character into array
        last_four_letters[i % 14] = char;
        // skip check if we don't have 4 letters yet
        if i < 14 {
            continue;
        };
        // check if array has only unique values
        if last_four_letters.iter().unique().count() == 14 {
            index = i + 1;
            break;
        }
    }

    println!("{}", index);
}
