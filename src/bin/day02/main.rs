use std::io::{self, Read};

use anyhow::{anyhow, Error};

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn main() -> Result<(), Error> {
    let commands = get_input()?;

    let (horizontal_position, depth) = part1(&commands);
    println!("{}", horizontal_position * depth);

    let (horizontal_position, depth) = part2(&commands);
    println!("{}", horizontal_position * depth);

    Ok(())
}

fn get_input() -> Result<Vec<Command>, Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let commands = buf
        .split('\n')
        .map(|s| {
            let mut iter = s.split(' ');

            let command_str = iter.next().unwrap();
            let command_num = iter.next().unwrap().parse()?;
            assert!(iter.next().is_none());

            match command_str {
                "forward" => Ok(Command::Forward(command_num)),
                "down" => Ok(Command::Down(command_num)),
                "up" => Ok(Command::Up(command_num)),
                _ => Err(anyhow!("invalid command string")),
            }
        })
        .collect::<Result<_, _>>()?;

    Ok(commands)
}

fn part1(commands: &[Command]) -> (usize, usize) {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Command::Forward(amount) => {
                horizontal_position += amount;
            }
            Command::Down(amount) => {
                depth += amount;
            }
            Command::Up(amount) => {
                depth -= amount;
            }
        }
    }

    (horizontal_position, depth)
}

fn part2(commands: &[Command]) -> (usize, usize) {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(amount) => {
                horizontal_position += amount;
                depth += aim * amount;
            }
            Command::Down(amount) => {
                aim += amount;
            }
            Command::Up(amount) => {
                aim -= amount;
            }
        }
    }

    (horizontal_position, depth)
}
