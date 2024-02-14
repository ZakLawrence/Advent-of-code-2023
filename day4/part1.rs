use std::fs::File;
use std::io::prelude::*;


fn read_file(file_path: &str) -> String{
    let mut file =  File::open(file_path).expect("File Not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    return data; 
}

fn game_score(winning_numbers: Vec<i32>, card_numbers: Vec<i32>) -> i32 {
    let mut score = 0;
    for num in card_numbers {
        if winning_numbers.contains(&num) {
            if score == 0 {
                score = 1;
            }
            else {
                score *= 2;
            }
        }
    }
    return score;
}

fn parse_data(data: String) {
    let mut total_score = 0;
    for line in data.lines() {
        let mut line_split = line.split([':','|']);
        let (_,wins,card) = (line_split.next().unwrap(),line_split.next().unwrap(),line_split.next().unwrap());
        //println!("Game: {}, Wins: {}, Card: {}",game,wins,card);
        let winning_numbers = wins.split_whitespace();
        let winning_numbers = winning_numbers.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let card_numbers = card.split_whitespace();
        let card_numbers = card_numbers.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        //println!("Game Score: {}",game_score(winning_numbers.clone(),card_numbers.clone()));
        total_score += game_score(winning_numbers,card_numbers);

    }
    println!("Total Score: {}",total_score);
}

fn main() {
    let data = read_file("input.txt");
    parse_data(data);
}