use std::io::{self, Read};

use anyhow::Error;

fn main() -> Result<(), Error> {
    let depths = get_input()?;

    let count = part1(&depths);
    println!("{}", count);

    let count = part2(&depths);
    println!("{}", count);

    Ok(())
}

fn get_input() -> Result<Vec<usize>, Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let depths = buf.split('\n').map(str::parse).collect::<Result<_, _>>()?;

    Ok(depths)
}

fn part1(depths: &[usize]) -> usize {
    let mut count = 0;

    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count += 1;
        }
    }

    count
}

fn part2(depths: &[usize]) -> usize {
    let mut count = 0;

    for i in 3..depths.len() {
        if (depths[i] + depths[i - 1] + depths[i - 2])
            > (depths[i - 1] + depths[i - 2] + depths[i - 3])
        {
            count += 1;
        }
    }

    count
}
