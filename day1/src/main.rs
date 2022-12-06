use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("./input").expect("NO");
    let input: Vec<&str> = contents.split_terminator("\n\n").collect();
    let input: Vec<Vec<&str>> = input
        .into_iter()
        .map(|s| s.split_terminator('\n').collect())
        .collect();

    let mut sums: Vec<usize> = Vec::new();
    for l in input {
        let mut sum: usize = 0;
        for s in l {
            let val = s.parse::<usize>().unwrap();
            sum += val;
        }
        sums.push(sum);
    }
    let max = sums.iter().max().unwrap();
    sums.sort();
    sums.reverse();
    println!("{}", sums[0] + sums[1] + sums[2])
}
