use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: &str) -> String{
    let mut file =  File::open(file_path).expect("File Not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    return data; 
}
 

fn parse_game(game_data: &str, num_red: i32, num_green: i32, num_blue: i32) -> (i32,bool){
    let parts = game_data.split([':',';']); 
    let mut current_game:i32 = 0;
    let mut valid_game:bool = true; 
    for part in parts{
        if part.contains("Game") {
            current_game = part.split_whitespace().last().expect("No Game Number Found!!").parse::<i32>().unwrap(); 
            valid_game = true;
        } else {
            let colours = part.split([',']);
            for colour in colours {
                let mut colour_itr = colour.split_whitespace();
                let (num_col,col) = (colour_itr.next().expect("").parse::<i32>().unwrap(), colour_itr.next().unwrap());
                if col == "red" && num_col > num_red {
                    valid_game = false;
                }
                if col == "green" && num_col > num_green {
                    valid_game = false;
                }
                if col == "blue" && num_col > num_blue {
                    valid_game = false;
                }
            }
            //println!("Current Game {}: {}, Valid Game: {}",current_game,part,valid_game);
        }
    }
    return (current_game,valid_game);    
}


fn parse_file(file_data: String, num_red: i32, num_green: i32, num_blue: i32){
    let mut total = 0; 
    for line in file_data.lines(){
        let (game,valid) = parse_game(line,num_red,num_green,num_blue);
        if valid{
            total += game;
        }
    }
    println!("Total: {}",total);
} 
 

fn main(){
    let (num_red,num_green,num_blue) = (12,13,14);
    let file_data = read_file("input.txt");
    parse_file(file_data,num_red,num_green,num_blue);
}