use std::{fs, time::Instant};

#[derive(Debug)]
struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Cubes>,
}

fn parse_input(input: &String) -> Vec<Game> {
    let mut result = vec![];

    for line in input.lines() {
        let id = line
            .split(":")
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let set_iter = line.split(":").nth(1).unwrap().split(";");

        let mut sets = vec![];

        for set_str in set_iter {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let cubes = set_str.split(",");

            for cube in cubes {
                let n = cube.split_whitespace().nth(0).unwrap().parse().unwrap();
                let color = cube.split_whitespace().nth(1).unwrap();

                match color {
                    "red" => red = n,
                    "green" => blue = n,
                    "blue" => green = n,
                    _ => panic!("something went wrong"), // Microsoft style error handling
                }
            }

            let set = Cubes { red, blue, green };
            sets.push(set);
        }

        let game = Game { id, sets };
        result.push(game);
    }

    return result;
}

fn solve_pt_2(input: &String) -> u32 {
    let games = parse_input(input);
    let mut sum = 0;

    for game in games {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;
        for set in game.sets {
            if set.red > min_red {
            min_red = set.red;
            }
            if set.blue > min_blue {
            min_blue = set.blue;
            }
            if set.green > min_green {
            min_green = set.green;
            }
        }

        sum += min_red * min_blue * min_green;
    }

    return sum;
    
}

fn solve_pt_1(input: &String) -> u32 {
    let games = parse_input(input);
    let mut possible_games = vec![];
    let red_limit = 12;
    let blue_limit = 13;
    let green_limit = 14;

    for game in games {
        let mut possible = true;
        for set in game.sets {
            if (set.red > red_limit) || (set.blue > blue_limit) || (set.green > green_limit) {
                possible = false;
            }
        }
        if possible {
            possible_games.push(game.id);
        }
    }
    possible_games.iter().sum()
}

fn main() {
    let input = fs::read_to_string("./input-2023-2").expect("Invalid file path");

    let test_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_string();

    let test_2 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();

    assert_eq!(solve_pt_1(&test_1), 8);
    assert_eq!(solve_pt_2(&test_2), 2286);

    let timer = Instant::now();

    let part_1 = solve_pt_1(&input);
    let part_2 = solve_pt_2(&input);

    println!("{:?}", timer.elapsed());

    assert_eq!(part_1, 2256);
    assert_eq!(part_2, 74229);

    println!("{part_1}");
    println!("{part_2}");
}
