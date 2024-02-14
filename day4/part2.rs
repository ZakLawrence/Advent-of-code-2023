use std::fs::File;
use std::io::prelude::*;


fn read_file(file_path: &str) -> String{
    let mut file =  File::open(file_path).expect("File Not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    return data; 
}

fn card_wins(winning_numbers: Vec<i32>, card_numbers: Vec<i32>) -> i32 {
    let mut score = 0;
    for num in card_numbers {
        if winning_numbers.contains(&num) {
            score += 1;
        }
    }
    return score;
}

fn generate_copies(mut cards: Vec<(i32,i32,i32)>) -> Vec<(i32,i32,i32)>{
    for card_index in 0..cards.len() {
        println!("Card: {}, Wins {}, Copies: {}",cards[card_index].0, cards[card_index].1, cards[card_index].2);
        for _copies in 1..=cards[card_index].2{
            for wins in 0..=cards[card_index].1{
                if card_index+wins as usize >= cards.len() {
                    println!("Break!");
                    break;
                }
                if wins != 0 {
                    cards[card_index+wins as usize].2 += 1;
                }
            }
        }
    }
    return cards;
}

fn string_to_vec(data: &str) -> Vec<i32>{
    let numbers = data.split_whitespace();
    let numbers = numbers.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    return numbers;
}

fn parse_data(data: String) {
    let mut total_cards = 0;
    let mut cards = Vec::new();
    for line in data.lines() {
        let mut line_split = line.split([':','|']);
        let (game,wins,card) = (line_split.next().unwrap(),line_split.next().unwrap(),line_split.next().unwrap());
        let game_number = game.split_whitespace().last().unwrap().parse::<i32>().unwrap();
        //println!("Game: {}, Wins: {}, Card: {}",game,wins,card);
        let winning_numbers = string_to_vec(wins);
        let card_numbers = string_to_vec(card);
        //println!("Game Score: {}",game_score(winning_numbers.clone(),card_numbers.clone()));
        let card_winnings = card_wins(winning_numbers,card_numbers);
        cards.push((game_number,card_winnings,1));
    }
    cards = generate_copies(cards);
    for card in cards {
        total_cards += card.2;
    }
    println!("Total Score: {}",total_cards);
}

fn main() {
    let data = read_file("input.txt");
    parse_data(data);
}