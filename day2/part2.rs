use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: &str) -> String{
    let mut file =  File::open(file_path).expect("File Not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    return data; 
}
 

fn parse_game(game_data: &str) -> i32{
    let parts = game_data.split([':',';']);
    let (mut red, mut green, mut blue) = (0,0,0); 
    for part in parts{
        if !part.contains("Game") {
            let colours = part.split([',']);
            for colour in colours {
                let mut colour_itr = colour.split_whitespace();
                let (num_col,col) = (colour_itr.next().expect("").parse::<i32>().unwrap(), colour_itr.next().unwrap());
                if col == "red" && num_col > red {
                    red = num_col;
                }
                if col == "green" && num_col > green {
                    green = num_col;
                }
                if col == "blue" && num_col > blue {
                    blue = num_col;
                }
            }
            //println!("Current Game {}: {}, Valid Game: {}",current_game,part,valid_game);
        }
    }
    return red*blue*green;    
}


fn parse_file(file_data: String){
    let mut total = 0; 
    for line in file_data.lines(){
        let game_score = parse_game(line);
        total += game_score;
    }
    println!("Total: {}",total);
} 
 

fn main(){
    let file_data = read_file("input.txt");
    parse_file(file_data);
}