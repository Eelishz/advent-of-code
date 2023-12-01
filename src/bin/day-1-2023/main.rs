use std::{collections::HashMap, fs};

fn print_byte_arr(x: &[u8]) {
    for c in x {
        print!("{}", *c as char);
    }
    println!("");
}

fn is_digit(line: &[u8], digits: &HashMap<&[u8], u8>) -> Option<u8> {
    // Expanding window matched to a map
    for i in 0..line.len() {
        let window = &line[i..line.len()];
        // print_byte_arr(window);
        if digits.contains_key(window) {
            return digits.get(window).copied();
        }
    }

    return None;
}

fn is_digit_reversed(line: &[u8], digits: &HashMap<&[u8], u8>) -> Option<u8> {
    // Expanding window matched to a map
    for i in 0..line.len() {
        let window = &line[0..line.len()-i];
        // print_byte_arr(window);
        if digits.contains_key(window) {
            return digits.get(window).copied();
        }
    }

    return None;
}

fn find_all_digits(line: &str, digits: &HashMap<&[u8], u8>) -> (u8, u8) {
    // Two pointers algorithm
    // Increment two pointers from left and right to find the first numbers

    let line_len = line.len();
    let mut l_ptr = 0;
    let mut r_ptr = line_len - 1;

    let mut l_digit = None;
    let mut r_digit = None;

    while !(l_digit.is_some() && r_digit.is_some()) {
        let l_char = &line.as_bytes()[0..l_ptr];
        let r_char = &line.as_bytes()[r_ptr..line_len];

        l_digit = is_digit(l_char, digits);
        r_digit = is_digit_reversed(r_char, digits);

        if !l_digit.is_some() {
            l_ptr += 1;
        }

        if !r_digit.is_some() {
            r_ptr -= 1;
        }
    }

    return (l_digit.unwrap(), r_digit.unwrap());
}

fn is_digit_ascii(c: u8) -> bool {
    (c >= 48) && (c <= 57)
}

fn find_numeric_digits(line: &str) -> (u8, u8) {
    // Two pointers algorithm
    // Increment two pointers from left and right to find the first numbers

    let line_len = line.len();
    let mut l_ptr = 0;
    let mut r_ptr = line_len - 1;

    let mut is_l_digit = false;
    let mut is_r_digit = false;

    while !(is_l_digit && is_r_digit) {
        let l_char = line.as_bytes()[l_ptr];
        let r_char = line.as_bytes()[r_ptr];

        is_l_digit = is_digit_ascii(l_char);
        is_r_digit = is_digit_ascii(r_char);

        if !is_l_digit {
            l_ptr += 1;
        }

        if !is_r_digit {
            r_ptr -= 1;
        }
    }

    
    let l_digit: u8 = (line.as_bytes()[l_ptr] as char).to_digit(10).unwrap() as u8;
    let r_digit: u8 = (line.as_bytes()[r_ptr] as char).to_digit(10).unwrap() as u8;
    
    return (l_digit, r_digit);
}

fn solve_pt_2(input: &String) -> u32 {
    let lines = input.lines();
    let mut sum = 0;

    let digits = HashMap::from([
        ("1".as_bytes(), 1),
        ("2".as_bytes(), 2),
        ("3".as_bytes(), 3),
        ("4".as_bytes(), 4),
        ("5".as_bytes(), 5),
        ("6".as_bytes(), 6),
        ("7".as_bytes(), 7),
        ("8".as_bytes(), 8),
        ("9".as_bytes(), 9),
        ("one".as_bytes(), 1),
        ("two".as_bytes(), 2),
        ("three".as_bytes(), 3),
        ("four".as_bytes(), 4),
        ("five".as_bytes(), 5),
        ("six".as_bytes(), 6),
        ("seven".as_bytes(), 7),
        ("eight".as_bytes(), 8),
        ("nine".as_bytes(), 9),
    ]);

    for line in lines {
        let digits = find_all_digits(line, &digits);
        sum += (digits.0 * 10) as u32 + digits.1 as u32;
    }

    return sum;
}

fn solve_pt_1(input: &String) -> u32 {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let digits = find_numeric_digits(line);
        sum += (digits.0 * 10) as u32 + digits.1 as u32;
    }

    return sum;
}

fn main() {
    let input = fs::read_to_string("./input-2023-1").expect("Invalid file path");
    let test_input_1 = String::from(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    );

    let test_input_2 = String::from(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    );

    let test_output_1 = 142;
    let test_output_2 = 281;

    assert_eq!(solve_pt_1(&test_input_1), test_output_1);
    assert_eq!(solve_pt_2(&test_input_2), test_output_2);

    let part_1 = solve_pt_1(&input);
    println!("{part_1}");

    let part_2 = solve_pt_2(&input);
    println!("{part_2}");
}
