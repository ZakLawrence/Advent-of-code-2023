use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: &str) -> String{
    let mut file =  File::open(file_path).expect("File Not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    return data; 
}

fn find_integers(line: &str) -> i32{
    let mut numbers:Vec<char> = Vec::new();
    for character in line.chars(){
        if character.is_numeric(){
            numbers.push(character)
        } 
    }
    let number:i32 = format!("{}{}",numbers[0],numbers[numbers.len() -1]).parse::<i32>().unwrap();
    return number;
}

fn process_data(file_data: String){
    let mut line_numbers:Vec<i32> = Vec::new();
    let mut total:i32 = 0; 
    for line in file_data.lines(){
        let line_number:i32 = find_integers(line);
        line_numbers.push(line_number);
        total += line_number;
    }
    println!("{}",total);
}

fn main(){
    let data = read_file("input.txt");
    process_data(data);
}