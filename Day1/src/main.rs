use std::path::Path;
use std::fs;

fn main() {
    let path = Path::new("input.txt");
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let depths = contents
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let count = depths
        .windows(3)// Added for part 2
        .map(|x| x.iter().sum())// Added for part 2
        .collect::<Vec<u32>>()// Added for part 2
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();

    println!("{}", count);
}
