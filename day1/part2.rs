use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: &str) -> String{
    let mut file =  File::open(file_path).expect("File Not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    return data; 
}

fn string_nums (line: &str) -> Vec<(usize,char)>{
    let mut numbers:Vec<(usize, char)> = Vec::new();
    let search_replace: Vec<(&str, char)> = vec![("one",'1'),("two",'2'),("three",'3'),("four",'4'),("five",'5'),("six",'6'),("seven",'7'),("eight",'8'),("nine",'9')];
    for (search,replace) in search_replace{
        let found:Vec<_> = line.match_indices(search).map(|(i,_)|i).collect();
        for find in found{
            numbers.push((find,replace));
        }
    } 
    return numbers;
}


fn find_integers(line: &str) -> i32{
    let mut numbers:Vec<(usize, char)> = Vec::new();
    for (index, character) in line.chars().enumerate(){
        if character.is_numeric(){
            numbers.push((index,character));
        } 
    }
    numbers.append(&mut string_nums(line));
    numbers.sort_by(|a,b| a.0.cmp(&b.0));
    let number:i32 = format!("{}{}",numbers[0].1,numbers[numbers.len() -1].1).parse::<i32>().unwrap();
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