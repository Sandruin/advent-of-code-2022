fn main() {
    let input = include_str!("../input.txt");

    // this solution is rather hacky and was fast to write
    // for the sake of my brain and rewritability it's propably much easier to write a proper structure,
    // even if that takes more overhead time

    // input is listed depth first

    // will hold name and size of dir
    // is a stack, which is fine since input is listed depth first
    let mut stack: Vec<(&str, u32)> = Default::default();
    let mut current_dir_name: &str = "/";
    let mut current_dir_size: u32 = 0;
    let mut dir_sizes_complete: Vec<(&str, u32)> = Default::default();

    // input doesn't go all the way back up, which is necessary for this recursion
    // so we simulate that with this hack:
    let mut depth = 0;

    for l in input.lines().skip(1) {
        // ignore dir statements
        if l.starts_with("dir") {
            continue;
        }
        // ignore ls statements
        if l == "$ ls" {
            continue;
        }
        println!("command: {l}");
        // match $ commands
        if l.starts_with("$") {
            println!("stack: {stack:?}");
            // add previous dir to stack
            if l == "$ cd .." {
                depth -= 1;
                // done with current element
                // check if size is big enough
                dir_sizes_complete.push((current_dir_name, current_dir_size));
                // update current element
                let current_element = stack.pop().unwrap();
                current_dir_name = current_element.0;
                // add child directory size on top
                current_dir_size = current_element.1 + current_dir_size;
            } else {
                depth += 1;
                println!("Pushing {current_dir_name}");
                stack.push((current_dir_name, current_dir_size));
                // match $ cd dirname
                current_dir_name = l.split_whitespace().last().unwrap();
                current_dir_size = 0;
            }
            continue;
        }
        // only files left
        let (num, _) = l.split_once(" ").unwrap();
        current_dir_size += num.parse::<u32>().unwrap_or_default();
    }

    // hack to finish recursion / go back up the tree / simulate $ cd ..
    for _ in 0..depth {
        // done with current element
        // check if size is big enough
        dir_sizes_complete.push((current_dir_name, current_dir_size));
        // update current element
        let current_element = stack.pop().unwrap();
        current_dir_name = current_element.0;
        // add child directory size on top
        current_dir_size = current_element.1 + current_dir_size;
    }

    // get sum of dirs smaller than 100_000
    let output = dir_sizes_complete
        .iter()
        .map(|(_, i)| i)
        .filter(|i| **i < 100_000)
        .sum::<u32>();

    println!("{:?}", output);
}
