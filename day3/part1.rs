use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct Point {
    x:i32,
    y:i32
}

impl Point{
    fn new(x:i32, y:i32)->Point{
        Point{
            x: x,
            y: y
        }
    }
}

impl PartialEq for Point{
    fn eq(&self, other: &Self) ->bool{
        return (self.x==other.x) && (self.y == other.y); 
    }
}

struct Part {
    value:i32,
    start_position:Point,
    neighbours:Vec<Point>,
    valid:bool
}

impl Part {
    fn find_neighbours(&self)->Vec<Point>{
        let mut adjacent = Vec::new();
        let lenght = self.value.to_string().len() as i32;
        let left = Point::new(self.start_position.x-1, self.start_position.y);
        let right = Point::new(self.start_position.x+lenght, self.start_position.y);
        for i in left.x..=right.x{
            let up = Point::new(i,self.start_position.y-1);
            let down = Point::new(i,self.start_position.y+1);
            adjacent.push(up);
            adjacent.push(down);
        }
        adjacent.push(left);
        adjacent.push(right);
        return adjacent;
    }

    fn set_neighburs(&mut self){
        self.neighbours = self.find_neighbours();
    }

    fn set_valid(&mut self, point:Point){
        if self.neighbours.iter().any(|&i| i == point){
            self.valid = true;
        }
    } 

    fn new(value:i32, position:Point)->Part{
        Part{
            value: value,
            start_position: position,
            neighbours: Vec::new(),
            valid: false   
        }
    }
}

fn read_file(file_path: &str) -> String{
    let mut file =  File::open(file_path).expect("File Not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
    return data; 
}

fn build_numbers(line_numbers:Vec<(usize,char)>)->Vec<(usize,i32)>{
    let mut return_numbers = Vec::new();
    if !line_numbers.is_empty() {
        let mut temp_string = String::new();
        let mut start_idx: usize = 0; 
        for (index,number) in line_numbers.iter().enumerate(){
            if index > 0 {
                if number.0 == line_numbers[index-1].0 + 1 {
                    temp_string += &number.1.to_string();
                } else {
                    let final_number = temp_string.parse::<i32>().unwrap();
                    return_numbers.push((start_idx,final_number));
                    start_idx = number.0;
                    temp_string = String::new(); 
                    temp_string += &number.1.to_string();
                }
            } else {
                temp_string += &number.1.to_string();
                start_idx = number.0;
            }
        }
        let final_number = temp_string.parse::<i32>().unwrap();
        return_numbers.push((start_idx,final_number));
    }
    return return_numbers;
}

fn build_parts(line_numbers:Vec<(usize,i32)>,line:usize) -> Vec<Part>{
    let mut parts = Vec::new();
    if !line_numbers.is_empty() {
        for  number in line_numbers{
            let mut part = Part::new(number.1,Point::new(number.0 as i32,line as i32));
            part.set_neighburs();
            parts.push(part);    
        }
    }
    return parts;
}


fn parse_input(input: String){
    let mut parts = Vec::new();
    let mut symbols = Vec::new();
    for (line_number, line) in input.lines().enumerate(){
        let mut line_numbers = Vec::new();
        for (index,character) in line.chars().enumerate(){
            if character.is_numeric(){
                line_numbers.push((index,character));
            } else if character != '.' {
                symbols.push(Point::new(index as i32,line_number as i32)); 
            }
        }
        if !line_numbers.is_empty() {
            let line_numbers = build_numbers(line_numbers);
            parts.append(&mut build_parts(line_numbers,line_number));
        }
    }
    for part in &mut parts{
        for symbol in symbols.iter(){
            part.set_valid(*symbol);
        }
    }
    let mut total = 0; 
    for part in parts{
        if part.valid{
            total+= part.value;
        }
    }    
    println!("Total: {}",total);
}


fn main(){
    let data = read_file("input.txt");
    parse_input(data);
}