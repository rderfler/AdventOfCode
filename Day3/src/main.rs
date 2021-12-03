#![feature(in_band_lifetimes)]
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let lines = contents.lines();
    let count: i32 = lines.clone().count().try_into().unwrap();
    let commands = lines
        .map(|x| { x
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
        });

    p1(commands.clone(), count);
    p2(commands);
}

fn p1(commands: impl Iterator<Item = Vec<u32>>, count: i32) {
    let command_bools = commands
        .reduce(|acc, x| acc
            .iter()
            .zip(x)
            .map(|x| x.0 + x.1)
            .collect::<Vec<u32>>())
        .unwrap()
        .iter()
        .map(|x| *x as i32)
        .map(|x| x * 2 - &count)
        .map(|x| x > 0)
        .collect::<Vec<bool>>();

    let binary = command_bools.clone().iter().map(|x| (*x as i32).to_string()).collect::<String>();
    let gamma = isize::from_str_radix(&binary, 2).unwrap();
    
    let binary = command_bools.iter().map(|x| !x).map(|x| (x as i32).to_string()).collect::<String>();
    let epsilon = isize::from_str_radix(&binary, 2).unwrap();

    println!("P1: {}", gamma * epsilon)
}

fn p2(commands: impl Iterator<Item = Vec<u32>>) {
    let collected_commands: Vec<Vec<u32>> = commands.collect::<Vec<Vec<u32>>>();
    let mut sub: Vec<Vec<u32>> = collected_commands.clone();
    let mut t: Vec<Vec<u32>>;
    let mut f: Vec<Vec<u32>>;

    let count = sub[0].len();
    for x in 0..count {
        if sub.len() == 1 {
            break;
        }
        t = Vec::new();
        f = Vec::new();

        for i in sub.into_iter() {
            if i[x] == 1 {
                t.push(i.clone());
            } else {
                f.push(i.clone());
            }
        }
        if t.len() >= f.len() {
            sub = t;
        } else {
            sub = f;
        }
    }
    let o2 = sub[0].iter().map(|x| x.to_string()).collect::<String>();

    
    let mut sub: Vec<Vec<u32>> = collected_commands;
    let mut t: Vec<Vec<u32>>;
    let mut f: Vec<Vec<u32>>;

    let count = sub[0].len();
    for x in 0..count {
        if sub.len() == 1 {
            break;
        }
        t = Vec::new();
        f = Vec::new();

        for i in sub.into_iter() {
            if i[x] == 0 {
                t.push(i.clone());
            } else {
                f.push(i.clone());
            }
        }
        if t.len() <= f.len() {
            sub = t;
        } else {
            sub = f;
        }
    }
    let co2 = sub[0].iter().map(|x| x.to_string()).collect::<String>();

    let o2_num = isize::from_str_radix(&o2, 2).unwrap();
    let co2_num = isize::from_str_radix(&co2, 2).unwrap();

    println!("P2: {}", o2_num * co2_num)
}