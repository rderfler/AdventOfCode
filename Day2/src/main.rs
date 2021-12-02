use std::fs;
use std::path::Path;
use std::str::FromStr;
use strum_macros::EnumString;

fn main(){
    let path = Path::new("input.txt");
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let commands = contents.lines().map(|x| {
        let split = x.split_whitespace().collect::<Vec<&str>>();
        Command {
            direction: Directions::from_str(split[0]).unwrap(),
            amount: split[1].parse().unwrap(),
        }
    });
    p1(commands.clone());
    p2(commands.clone());
}

fn p1(commands: impl Iterator<Item = Command>) {
    let (x, z) = commands.fold((0, 0), |acc, x| x.traverse_p1(acc));

    println!("Part1");
    println!("x = {}, z = {}, x * z = {}", x, z, x * z);
}

fn p2(commands: impl Iterator<Item = Command>){
    let (x, z, aim) = commands.fold((0, 0, 0), |acc, x| x.traverse_p2(acc));

    println!("Part2");
    println!("x = {}, z = {}, x * z = {}", x, z, x * z);

}

struct Command {
    direction: Directions,
    amount: i32,
}

impl Command {
    pub fn traverse_p1(&self, coordinates: (i32, i32)) -> (i32, i32) {
        match self.direction {
            Directions::Forward => (coordinates.0 + self.amount, coordinates.1),
            Directions::Down => (coordinates.0, coordinates.1 + self.amount),
            Directions::Up => (coordinates.0, coordinates.1 - self.amount),
        }
    }

    pub fn traverse_p2(&self, coordinates: (i32, i32, i32)) -> (i32, i32, i32) {
        match self.direction {
            Directions::Forward => (
                coordinates.0 + self.amount, 
                coordinates.1 + self.amount * coordinates.2, 
                coordinates.2),
            Directions::Down => (
                coordinates.0, 
                coordinates.1, 
                coordinates.2 + self.amount),
            Directions::Up => (
                coordinates.0, 
                coordinates.1, 
                coordinates.2 - self.amount),
        }
    }
}

#[derive(EnumString)]
enum Directions {
    #[strum(ascii_case_insensitive)]
    Forward,
    #[strum(ascii_case_insensitive)]
    Down,
    #[strum(ascii_case_insensitive)]
    Up,
}
