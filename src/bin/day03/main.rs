use std::io::{self, Read};

use anyhow::Error;

fn main() -> Result<(), Error> {
    let (numbers, precision) = get_input()?;

    let (gamma_rate, epsilon_rate) = part1(&numbers, precision);
    println!("{}", gamma_rate * epsilon_rate);

    let (oxygen_generator_rating, co2_scrubber_rating) = part2(&numbers, precision);
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);

    Ok(())
}

fn get_input() -> Result<(Vec<usize>, usize), Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let numbers = buf
        .split('\n')
        .map(|s| usize::from_str_radix(s, 2))
        .collect::<Result<_, _>>()?;
    let precision = buf.split('\n').next().unwrap().len();

    Ok((numbers, precision))
}

fn part1(numbers: &[usize], precision: usize) -> (usize, usize) {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..precision {
        let mut count = 0;

        for number in numbers {
            if number & 1 << i != 0 {
                count += 1;
            } else {
                count -= 1;
            }
        }

        if count >= 0 {
            gamma_rate += 1 << i;
        } else {
            epsilon_rate += 1 << i;
        }
    }

    (gamma_rate, epsilon_rate)
}

fn part2(numbers: &[usize], precision: usize) -> (usize, usize) {
    let oxygen_generator_rating =
        get_rating(numbers, precision, true).expect("could not get an oxygen generator rating");
    let co2_scrubber_rating =
        get_rating(numbers, precision, false).expect("could not get an co2 scrubber rating");

    (oxygen_generator_rating, co2_scrubber_rating)
}

fn get_rating(numbers: &[usize], precision: usize, is_most_common: bool) -> Option<usize> {
    let mut candidates = numbers.to_vec();

    for i in (0..precision).rev() {
        if candidates.len() == 1 {
            break;
        }

        let mut count = 0;

        for number in &candidates {
            if number & 1 << i != 0 {
                count += 1;
            } else {
                count -= 1;
            }
        }

        candidates.retain(|n| (n & 1 << i != 0) ^ (count >= 0) ^ is_most_common);
    }

    if candidates.len() == 1 {
        Some(candidates[0])
    } else {
        None
    }
}
