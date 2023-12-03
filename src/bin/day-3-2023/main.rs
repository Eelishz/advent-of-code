use std::{fs, time::Instant};

#[derive(Debug)]
struct Number {
    value: u32,
    counted: bool,
}

fn is_digit_ascii(c: char) -> bool {
    (c as u8 >= 48) && (c as u8 <= 57)
}

fn is_symbol(c: char) -> bool {
    // Not digit and not dot (.)
    (!is_digit_ascii(c)) && (c != '.')
}

fn is_gear(c: char) -> bool {
    c  == '*'
}

fn parse_input(input: &String, predicate: fn(char) -> bool) -> (Vec<(usize, usize)>, Vec<Vec<Option<u32>>>) {
    let mut symbols = vec![];
    let mut number_matrix = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut number_row = vec![];

        for (j, c) in line.chars().enumerate() {
            if predicate(c) {
                symbols.push((i, j));
            }
            if is_digit_ascii(c) {
                let digit = c.to_digit(10).unwrap() as u32;
                number_row.push(Some(digit));
            } else {
                number_row.push(None);
            }
        }
        number_row.push(None); // padding

        number_matrix.push(number_row);
    }

    return (symbols, number_matrix);
}

fn print_numbers(numbers_matrix: &Vec<Vec<Option<usize>>>, numbers: &Vec<Number>) {
    fn left_pad(input: String, l: usize) -> String {
        let mut buf = String::new();
        let pad = l - input.len();

        buf.push_str(&" ".repeat(pad));
        buf.push_str(&input);

        return buf;
    }

    for row in numbers_matrix {
        for i in row {
            if let Some(i) = *i {
                let n = numbers[i].value;
                print!("{} ", left_pad(n.to_string(), 3))
            } else {
                print!("{} ", left_pad(".".to_string(), 3));
            }
        }
        println!("");
    }
}

fn middle_part(digit_matrix: Vec<Vec<Option<u32>>>) -> (Vec<Number>, Vec<Vec<Option<usize>>>) {
    // Not to be confused with the middle-out algorithm

    let x_len = digit_matrix[0].len();
    let y_len = digit_matrix.len();

    let mut number_matrix = vec![vec![None; x_len]; y_len];
    let mut numbers = vec![];

    for i in 0..y_len {
        let mut digits = vec![];

        for j in 0..x_len {
            if let Some(n) = digit_matrix[i][j] {
                digits.push(n);
            } else {
                let mut number = 0;
                for (k, digit) in digits.iter().rev().enumerate() {
                    let place = 10_u32.pow(k as u32);
                    number += digit * place;
                }
                if number == 0 {
                    continue;
                }
                numbers.push(Number {
                    value: number,
                    counted: false,
                });
                for k in 0..digits.len() {
                    number_matrix[i][j - k - 1] = Some(numbers.len() - 1);
                }

                digits = vec![];
            }
        }
    }

    return (numbers, number_matrix);
}

fn solve_pt_2(input: &String) -> u32 {
    let predicate = is_gear;
    let (symbols, digit_matrix) = parse_input(input, predicate);
    
    let x_len = digit_matrix[0].len();
    let y_len = digit_matrix.len();

    let (mut numbers, number_matrix) = middle_part(digit_matrix);

    let mut sum = 0;

    for (i, j) in symbols {
        let (i, j) = (i as i32, j as i32);
        
        let mut number_a = 0;
        let mut number_b = 0;

        for x in -1 + i..2 + i {
            for y in -1 + j..2 + j {
                let x = x.clamp(0, x_len as i32 - 1) as usize;
                let y = y.clamp(0, y_len as i32 - 1) as usize;
                if let Some(number_index) = number_matrix[x][y] {
                    let number = &mut numbers[number_index];
                    if !number.counted {
                        if number_a == 0 {
                            number_a = number.value;
                        } else {
                            number_b = number.value;
                        }
                        number.counted = true;
                    }
                }
            }
        }

        sum += number_a * number_b;
    }

    return sum;
}

fn solve_pt_1(input: &String) -> u32 {
    let predicate = is_symbol;
    let (symbols, digit_matrix) = parse_input(input, predicate);
    
    let x_len = digit_matrix[0].len();
    let y_len = digit_matrix.len();

    let (mut numbers, number_matrix) = middle_part(digit_matrix);

    // print_numbers(&number_matrix, &numbers);

    let mut sum = 0;

    for (i, j) in symbols {
        let (i, j) = (i as i32, j as i32);
        for x in -1 + i..2 + i {
            for y in -1 + j..2 + j {
                let x = x.clamp(0, x_len as i32 - 1) as usize;
                let y = y.clamp(0, y_len as i32 - 1) as usize;
                if let Some(number_index) = number_matrix[x][y] {
                    let number = &mut numbers[number_index];
                    if !number.counted {
                        sum += number.value;
                        number.counted = true;
                    }
                }
            }
        }
    }

    return sum;
}

fn main() {
    let input = fs::read_to_string("./input-2023-3").expect("Invalid file path");

    let test_1 = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();

    let test_2 = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();

    assert_eq!(solve_pt_1(&test_1), 4361);
    assert_eq!(solve_pt_2(&test_2), 467835);

    let timer = Instant::now();

    let part_1 = solve_pt_1(&input);
    let part_2 = solve_pt_2(&input);

    println!("{:?}", timer.elapsed());

    assert_eq!(part_1, 543867);
    assert_eq!(part_2, 79613331);
    
    println!("{part_1}");
    println!("{part_2}");
}
