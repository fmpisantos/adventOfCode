use std::{fs::File, io::{Read, self, BufRead}};

use crate::my_types::{Card, Matrix, Numbers, EnginePart};

fn get_path(test: bool) -> String {
    let mut path = String::from("src/inputs/");
    if test {
        path += "test";
    } else {
        path += "day";
    }
    return path;
}

pub fn read_input_as_cards(day: &str, test: bool) -> Card {
    return read_file_as_numbers(&format!("{}{}", get_path(test), day)).unwrap();
} 

pub fn read_input_as_matrix(day: &str, test: bool) -> Matrix {
    return read_file_as_matrix(&format!("{}{}", get_path(test), day)).unwrap();    
}

fn read_file_as_numbers(file_path: &String) -> io::Result<Card> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut numbers: Card = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let split_line = &mut line.split("|");
        let mut num: Numbers = Numbers::default();
        if let Some(nums) = &mut split_line.next() {
            let mine_split = &mut nums.split(":");
            if let Some(card) = &mine_split.next() {
                let mut card_as_str: String = String::new();
                for char in card.chars() {
                    if char.is_digit(10) {
                        card_as_str.push(char);
                    }
                }
                num.card_nr = card_as_str.parse().unwrap_or(0);
            }
            if let Some(mine) = mine_split.next() {
                Numbers::parse_nums(&mut num.mine, mine);
            }
        }
        if let Some(winning) = split_line.next() {
            Numbers::parse_nums(&mut num.winning, winning);
        }
        numbers.push(num);
    }

    Ok(numbers)
}

fn read_file_as_matrix(file_path: &String) -> io::Result<Matrix> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut matrix: Matrix = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<EnginePart> = line.chars().map(|ch| EnginePart::new(ch)).collect();
        matrix.push(row);
    }

    Ok(matrix)
}

pub fn read_input_as_string(day: &str, test: bool) -> String {
    return read_string_from_file(&format!("{}{}", get_path(test), day));
}

fn read_string_from_file(filename: &String) -> String {
    let file_result = File::open(filename);

    match file_result {
        Ok(mut file) => {
            let mut buffer = String::new();

            let read_result = file.read_to_string(&mut buffer);
            match read_result {
                Ok(_) => {
                    return buffer;
                }
                Err(_) => {
                    return String::new();
                }
            }
        }
        Err(_) => {
            return String::new();
        }
    }
}

