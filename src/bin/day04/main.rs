use std::io::{self, Read};
use std::iter;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let (numbers, boards) = get_input()?;

    if let Some(n) = part1(&numbers, &boards) {
        println!("{}", n);
    }

    if let Some(n) = part2(&numbers, &boards) {
        println!("{}", n);
    }

    Ok(())
}

fn get_input() -> Result<(Vec<u8>, Vec<[[u8; 5]; 5]>), Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut iter = buf.split("\n\n");

    let numbers = iter
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let boards = iter
        .map(|board_str| {
            let mut board = [[0; 5]; 5];

            for (row, row_str) in iter::zip(&mut board, board_str.split('\n')) {
                for (num, num_str) in iter::zip(row, row_str.split_whitespace()) {
                    *num = num_str.parse()?;
                }
            }

            Ok::<_, Error>(board)
        })
        .collect::<Result<_, _>>()?;

    Ok((numbers, boards))
}

fn part1(numbers: &[u8], boards: &[[[u8; 5]; 5]]) -> Option<usize> {
    let mut boards = add_marks_to_boards(boards);

    for number in numbers {
        for board in &mut boards {
            mark_number(*number, board);

            if check_won(board) {
                let mut sum = sum_unmarked(board);
                sum *= *number as usize;
                return Some(sum);
            }
        }
    }

    None
}

fn part2(numbers: &[u8], boards: &[[[u8; 5]; 5]]) -> Option<usize> {
    let mut boards = add_marks_to_boards(boards);

    for number in numbers {
        for board in &mut boards {
            mark_number(*number, board);
        }

        if boards.len() != 1 {
            boards.retain(|board| {!check_won(board)});
        } else {
            let board = boards[0];
            if check_won(&board) {
                let mut sum = sum_unmarked(&board);
                sum *= *number as usize;
                return Some(sum);
            }
        }
    }

    None
}

fn add_marks_to_boards(boards: &[[[u8; 5]; 5]]) -> Vec<[[(u8, bool); 5]; 5]> {
    let mut new_boards = vec![[[(0, false); 5]; 5]; boards.len()];

    for (board, new_board) in iter::zip(boards, &mut new_boards) {
        for (row, new_row) in iter::zip(board, new_board) {
            for (num, (new_num, _)) in iter::zip(row, new_row) {
                *new_num = *num;
            }
        }
    }

    new_boards
}

fn mark_number(number: u8, board: &mut [[(u8, bool); 5]; 5]) {
    for row in board {
        for (num, mark) in row {
            if *num == number {
                *mark = true;
            }
        }
    }
}

fn check_won(board: &[[(u8, bool); 5]; 5]) -> bool {
    'rows: for i in 0..5 {
        for j in 0..5 {
            if let (_, false) = board[i][j] {
                continue 'rows;
            }
        }

        return true;
    }

    'columns: for i in 0..5 {
        for j in 0..5 {
            if let (_, false) = board[j][i] {
                continue 'columns;
            }
        }

        return true;
    }

    false
}

fn sum_unmarked(board: &[[(u8, bool); 5]; 5]) -> usize {
    let mut sum = 0;

    for row in board {
        for (num, mark) in row {
            if !*mark {
                sum += *num as usize;
            }
        }
    }

    sum
}
