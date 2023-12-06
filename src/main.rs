use regex::{Captures, Regex};
use std::fs;

fn main() {
    //process input
    let day_one_data = fs::read_to_string("./assets/d1.txt").unwrap();
    let day_two_data = fs::read_to_string("./assets/d2.txt").unwrap();
    let day_three_data = fs::read_to_string("./assets/d3.txt").unwrap();
    let day_four_data = fs::read_to_string("./assets/d4.txt").unwrap();

    //solve
    let one_one = day_one_one(&day_one_data.as_str());
    let one_two = day_one_two(&day_one_data.as_str());
    let two_one = day_two_one(&day_two_data.as_str());
    let two_two = day_two_two(&day_two_data.as_str());
    let three_one = day_three_one(&day_three_data.as_str());
    let three_two = day_three_two(&day_three_data.as_str());
    let four_one = day_four_one(&day_four_data.as_str());
    let four_two = day_four_two(&day_four_data.as_str());

    //print
    println!("day 1.1: {}", one_one);
    println!("day 1.2: {}", one_two);
    println!("day 2.1: {}", two_one);
    println!("day 2.2: {}", two_two);
    println!("day 3.1: {}", three_one);
    println!("day 3.2: {}", three_two);
    println!("day 4.1: {}", four_one);
    println!("day 4.2: {}", four_two);
}

fn day_one_one(input: &str) -> u32 {
    let lines = input.split("\n");
    let mut total = 0;
    for line in lines {
        let numbers = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<char>>();
        total +=
            numbers[0].to_digit(10).unwrap() * 10 + numbers[numbers.len() - 1].to_digit(10).unwrap()
    }
    total
}

fn day_one_two(input: &str) -> u32 {
    let lines = input.split("\n");
    let mut total = 0;
    for line in lines {
        let re = Regex::new("(zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let edited_line = re.replace_all(line, |cap: &Captures| {
            match &cap[0] {
                "zero" => "0",
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => panic!("We should never get here"),
            }
            .to_string()
        });
        let numbers = edited_line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<char>>();
        total +=
            numbers[0].to_digit(10).unwrap() * 10 + numbers[numbers.len() - 1].to_digit(10).unwrap()
    }
    total
}

fn day_two_one(input: &str) -> i32 {
    let re = Regex::new("(,|;|:)").unwrap();
    let sanitized = re.replace_all(input, |cap: &Captures| {
        match &cap[0] {
            "," => "",
            ";" => "",
            ":" => "",
            _ => panic!("We should never get here"),
        }
        .to_string()
    });
    let games = sanitized.split("\n");
    let mut id_total = 0;
    for game in games {
        let words = game.split(" ").collect::<Vec<&str>>();
        if words.len() > 2 {
            let game_id = words[1].parse::<i32>().unwrap();
            let mut green = 0;
            let mut red = 0;
            let mut blue = 0;

            let mut current = 0;

            for j in 2..words.len() {
                if j % 2 == 0 {
                    current = words[j].parse::<i32>().unwrap();
                } else {
                    if words[j].trim() == "red" && current > red {
                        red = current;
                        current = 0;
                    } else if words[j].trim() == "blue" && current > blue {
                        blue = current;
                        current = 0;
                    } else if words[j].trim() == "green" && current > green {
                        green = current;
                        current = 0;
                    }
                }
            }

            if red <= 12 && green <= 13 && blue <= 14 {
                id_total += game_id;
            }
        }
    }
    id_total
}

fn day_two_two(input: &str) -> i32 {
    let re = Regex::new("(,|;|:)").unwrap();
    let sanitized = re.replace_all(input, |cap: &Captures| {
        match &cap[0] {
            "," => "",
            ";" => "",
            ":" => "",
            _ => panic!("We should never get here"),
        }
        .to_string()
    });
    let games = sanitized.split("\n");
    let mut power_sum = 0;
    for game in games {
        let words = game.split(" ").collect::<Vec<&str>>();
        if words.len() > 2 {
            let mut green = 0;
            let mut red = 0;
            let mut blue = 0;

            let mut current = 0;

            for j in 2..words.len() {
                if j % 2 == 0 {
                    current = words[j].parse::<i32>().unwrap();
                } else {
                    if words[j].trim() == "red" && current > red {
                        red = current;
                        current = 0;
                    } else if words[j].trim() == "blue" && current > blue {
                        blue = current;
                        current = 0;
                    } else if words[j].trim() == "green" && current > green {
                        green = current;
                        current = 0;
                    }
                }
            }

            let power = red * blue * green;
            power_sum += power;
        }
    }
    power_sum
}

fn day_three_one(input: &str) -> i32 {
    let rows = input.split("\n").collect::<Vec<&str>>();
    let row_len = rows[0].len();
    let col_len = rows.len();
    let mut total = 0;
    for y in 0..col_len {
        let row = rows[y].chars().collect::<Vec<char>>();
        for x in 0..row_len {
            let char = row[x];
            if !char.is_alphanumeric() && char != '.' {
                let tl = check_coor_for_num(&rows, x - 1, y - 1);
                let tm = check_coor_for_num(&rows, x, y - 1);
                let tr = check_coor_for_num(&rows, x + 1, y - 1);
                total += tl;
                if tm != tl {
                    total += tm;
                }
                if tr != tm {
                    total += tr;
                }

                let ml = check_coor_for_num(&rows, x - 1, y);
                let mr = check_coor_for_num(&rows, x + 1, y);
                total += ml;
                if mr != ml {
                    total += mr;
                }

                let bl = check_coor_for_num(&rows, x - 1, y + 1);
                let bm = check_coor_for_num(&rows, x, y + 1);
                let br = check_coor_for_num(&rows, x + 1, y + 1);
                total += bl;
                if bm != bl {
                    total += bm;
                }
                if br != bm {
                    total += br;
                }
            }
        }
    }
    total
}

//todo: fix it
fn day_three_two(input: &str) -> i32 {
    let rows = input.split("\n").collect::<Vec<&str>>();
    let row_len = rows[0].len();
    let col_len = rows.len();
    let mut total = 0;
    for y in 0..col_len {
        let row = rows[y].chars().collect::<Vec<char>>();
        for x in 0..row_len {
            let char = row[x];
            if char == '*' {
                let mut ratio = 1;
                let mut part_count = 0;
                let tl = check_coor_for_num(&rows, x - 1, y - 1);
                let tm = check_coor_for_num(&rows, x, y - 1);
                let tr = check_coor_for_num(&rows, x + 1, y - 1);
                if tl != 0 {
                    ratio *= tl;
                    part_count += 1;
                }
                if tm != tl && tm != 0 {
                    ratio *= tm;
                    part_count += 1;
                }
                if tr != tm && tr != 0 {
                    ratio *= tr;
                    part_count += 1;
                }

                let ml = check_coor_for_num(&rows, x - 1, y);
                let mr = check_coor_for_num(&rows, x + 1, y);
                if ml != 0 {
                    ratio *= ml;
                    part_count += 1;
                }
                if mr != ml && mr != 0 {
                    ratio *= mr;
                    part_count += 1;
                }

                let bl = check_coor_for_num(&rows, x - 1, y + 1);
                let bm = check_coor_for_num(&rows, x, y + 1);
                let br = check_coor_for_num(&rows, x + 1, y + 1);
                if bl != 0 {
                    ratio *= bl;
                    part_count += 1;
                }
                if bm != bl && bm != 0 {
                    ratio *= bm;
                    part_count += 1;
                }
                if br != bm && br != 0 {
                    ratio *= br;
                    part_count += 1;
                }
                if part_count == 2 {
                    total += ratio;
                }
            }
        }
    }
    total
}

fn check_coor_for_num(grid: &Vec<&str>, x: usize, y: usize) -> i32 {
    if y >= grid.len() || x >= grid[0].chars().collect::<Vec<char>>().len() {
        return 0;
    }
    let row = grid[y].chars().collect::<Vec<char>>();
    if row[x].is_numeric() {
        let mut start_of_num = x;
        for i in (0..x).rev() {
            if row[i].is_numeric() {
                start_of_num = i
            } else {
                break;
            }
        }
        let mut number_string = String::from("");
        for i in start_of_num..row.len() {
            if row[i].is_numeric() {
                number_string.push(row[i]);
            } else {
                break;
            }
        }
        return number_string.parse().unwrap();
    }
    0
}

fn day_four_one(input: &str) -> i32 {
    let cards = input.split("\n");
    let mut total = 0;
    for card in cards {
        let colon_index = card.find(":").unwrap();
        let pipe_index = card.find("|").unwrap();
        let win_num = &card[colon_index..pipe_index].trim();
        let game_num = &card[pipe_index..].trim();

        let mut score = 0;
        for wnum in win_num.split(" ") {
            for gnum in game_num.split(" ") {
                if wnum == gnum && wnum != "" {
                    if score == 0 {
                        score = 1;
                    } else {
                        score += score;
                    }
                }
            }
        }
        total += score;
    }
    total
}

fn day_four_two(input: &str) -> i32 {
    let cards = input.split("\n").collect::<Vec<&str>>();
    let mut card_copies_vec = vec![1; cards.len()];
    let mut card_wins_vec = vec![0; cards.len()];
    for i in 0..cards.len() {
        let card = cards[i];
        let colon_index = card.find(":").unwrap();
        let pipe_index = card.find("|").unwrap();
        let win_num = &card[colon_index..pipe_index].trim();
        let game_num = &card[pipe_index..].trim();

        for wnum in win_num.split(" ") {
            for gnum in game_num.split(" ") {
                if wnum == gnum && wnum != "" {
                    card_wins_vec[i] += 1;
                }
            }
        }
    }
    for i in 0..card_copies_vec.len() {
        let copies = card_copies_vec[i];
        for j in 0..card_wins_vec[i] {
            let index = i + 1 + j;
            if j < card_copies_vec.len() {
                card_copies_vec[index] += copies;
            }
        }
    }
    card_copies_vec.iter().sum()
}
