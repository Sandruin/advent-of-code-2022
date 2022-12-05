use itertools::Itertools;

fn main() {
    let input = include_bytes!("../input.txt");

    // split at newline
    let output = input
        .split(|s| *s == b'\n')
        // get tuple of halved line (both compartments)
        .map(|rs| rs.split_at(rs.len() / 2))
        .map(|(c1, c2)| {
            c1
                // iterate over first compartment
                .iter()
                // yield c1 char if it is in c2
                .filter(|c1| c2.contains(c1))
                // change to priority
                .map(|c1| {
                    if *c1 >= b'a' {
                        (c1 - b'a') as i16 + 1
                    } else {
                        (c1 - b'A') as i16 + 27
                    }
                })
                // count multiple mentions just once
                .unique()
                // sum over all items that are in left and right compartment
                .sum::<i16>()
        })
        // sum over all rucksacks
        .sum::<i16>();

    println!("{}", output);
}
