use crate::my_types::{EnginePart, Matrix, VecPart, IsDigit, NewTrait};

pub fn pow(num: usize, base: usize) -> usize {
    let mut result = 1;
    for _ in 0..num {
        result *= base;
    }
    result
}

pub fn get_first_digit(line: &mut Vec<EnginePart>, reversed: bool, consider_string: bool) -> u32 {
    let num = get_first_digits(line, reversed, false, consider_string);
    return num;
}

fn get_num_from_string(str_value: &String) -> u8 {
    if str_value.contains("zero") {
        return 0;
    }
    if str_value.contains("one") {
        return 1;
    }
    if str_value.contains("two") {
        return 2;
    }
    if str_value.contains("three") {
        return 3;
    }
    if str_value.contains("four") {
        return 4;
    }
    if str_value.contains("five") {
        return 5;
    }
    if str_value.contains("six") {
        return 6;
    }
    if str_value.contains("seven") {
        return 7;
    }
    if str_value.contains("eight") {
        return 8;
    }
    if str_value.contains("nine") {
        return 9;
    }
    return 10;
}

fn get_first_digits(line: &mut VecPart, reversed: bool, multiple: bool, consider_string: bool) -> u32 {
    return _get_first_digits(line, 0, line.len(), reversed, multiple, consider_string, &mut 0);
}

fn _get_first_digits(line: &mut Vec<EnginePart>, min: usize, max: usize, reversed: bool, multiple: bool, consider_string: bool, zeros_before: &mut usize) -> u32 {
    let indexes = if reversed {
        (min..max).rev().collect::<Vec<_>>()
    } else {
        (min..max).collect::<Vec<_>>()
    };

    let mut number = String::new();
    let mut found = false;
    let mut str_rep = String::new();
    for i in indexes {
        //print!(" -- {} -- {} --", i, line[i].ch);
        if line[i].is_digit(10) {
            if reversed {
                number.insert(0, line[i].get_if_not_seen());
            } else {
                number.insert(number.len(), line[i].get_if_not_seen());
            }
            line[i].see();
            found = true;
            if !multiple {
                return line[i].to_digit(10).unwrap_or_default();
            }
        } else  if !line[i].seen() {
            if number.len() == 0 {
                return 0;
            }
            return get_u32_from_str(&number, zeros_before);
        } else {
            if found {
                return get_u32_from_str(&number, zeros_before);
            }else if consider_string  {
                if reversed {
                    str_rep.insert(0,line[i].ch);
                } else {
                    str_rep.insert(str_rep.len(),line[i].ch);
                }
                let num = get_num_from_string(&str_rep); 
                if num < 10 {
                    return num as u32;
                } else if str_rep.len() >= 5 {
                    if reversed {
                        str_rep = String::from(&str_rep[..4]);
                    } else {
                        str_rep = String::from(&str_rep[1..]);
                    }
                } 
            }
        }
    }
    if number.len() == 0 {
        return 0;
    }
    return get_u32_from_str(&number, zeros_before);
}

fn get_color_index(color: &str) -> usize {
    match color {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => {panic!("wrong color")}
    }
}

fn get_u32_from_str(number: &String, zeros_before: &mut usize) -> u32 {
    //print!("got: {}", number);
    for char in number.chars() {
        if char != '0' {
            break;
        }
        *zeros_before = zeros_before.wrapping_add(1);
    }
    match number.parse::<u32>() {
        Ok(number) => {
            return number
        }
        Err(err) => {panic!("{}", err)}
    }
}

pub fn validate_game(game: Vec<&str>, rgb: &[u32;3]) -> u32 {
    for hand in game.get(1).unwrap_or(&"").split(";") {
        //println!("{}", hand);
        for color in hand.split(",") {
            let mut to_check = color.trim().split(" ");
            let col = to_check.nth(1).unwrap_or_default();
            let num = get_first_digits(&mut color.chars().map(|ch| EnginePart::new(ch)).collect::<Vec<_>>(), false, true, false);
            let col_idx = get_color_index(col);
            //println!("{}{}{}", col_idx, col, num);
            if rgb[get_color_index(col)] < num {
                return 0; 
            }
        }
    }
    return get_first_digits(&mut game.get(0).unwrap_or(&"").chars().map(|ch| EnginePart::new(ch)).collect::<Vec<_>>(), true, true, false);
}


pub fn min_rgb(game: Vec<&str>) -> [u32;3] {
    let mut rgb: [u32;3] = [0,0,0]; 
    for hand in game.get(1).unwrap_or(&"").split(";") {
        //println!("{}", hand);
        for color in hand.split(",") {
            let mut to_check = color.trim().split(" ");
            let col = to_check.nth(1).unwrap_or_default();
            let num = get_first_digits(&mut color.chars().map(|ch| EnginePart::new(ch)).collect(), false, true, false);
            let col_idx = get_color_index(col);
            //println!("{}{}{}", col_idx, col, num);
            if rgb[col_idx] < num {
                rgb[col_idx] = num;
            }
        }
    }

    return rgb;
}

fn get_while_digit(line: &mut Vec<EnginePart>, x: usize, rev: bool) -> String {
    if x >= line.len() || !line[x].is_digit(10) {
        return String::new(); 
    } 
    if rev {
        return get_while_digit(line, x.wrapping_sub(1), rev) + &line[x].see().to_string();
    }
    return line[x].see().to_string() + &get_while_digit(line, x.wrapping_add(1), rev);
}

fn _get_number_in_pos(line: &mut Vec<EnginePart>, x: usize) -> u32 {
    if x >= line.len() || !line[x].is_digit(10) || line[x].seen() {
        return 0;
    }
    //print!(" x: {} ({})", x, line[x].ch);
    let value: String = get_while_digit(line, x, true) + &get_while_digit(line, x.wrapping_add(1), false); 
   
    //print!(" -> {}", value);
    return value.parse::<u32>().unwrap_or(0);
}

fn get_number_in_pos(line: &mut Vec<EnginePart>, x: usize) -> u32 {
    if x >= line.len() {
        return 0;
    }
    //print!(" x: {} ({})", x, line[x].ch);
    if !line[x].is_digit(10) {
        return 0;
    }
    let mut value = String::new();
    if x > 0 && line[x.wrapping_sub(1)].is_digit(10) {
        value += &_get_first_digits(line, 0, x, true, true, false, &mut 0).to_string();
    }
    let mut zeros: usize = 0;
    let to_add = &_get_first_digits(line, x, line.len(), false, true, false, &mut zeros).to_string();
    //print!(" zeros: {} ", zeros);
    value += &"0".repeat(zeros);
    value += to_add; 
    //print!(" = {}", value);
    return value.parse::<u32>().unwrap_or(0);
}

fn get_num_in_pos(line: &mut Vec<EnginePart>, x: usize) -> Option<i32> {
    let value = &line[x]; 
    if value.seen() || !value.is_digit(10) {
        return None;
    }
    return Some(_get_number_in_pos(line, x) as i32);
}

fn mult_surrounding_nums_line(x: usize, y: usize, matrix: &mut Matrix) -> i32 {
    let mut value: i32 = 1;
    let mut flag = false;
    //print!("y: {}",y);
    if x != 0 {
       if let Some(aux) = get_num_in_pos(&mut matrix[y], x.wrapping_sub(1)) {
            flag = true;
            value *= aux;
       } 
    }

    if flag && matrix[y][x].is_digit(10) {
        if let Some(aux) = get_num_in_pos(&mut matrix[y], x) {
            return aux;
        }
    }
    
    if x.wrapping_add(1) < matrix[y].len() {
       if let Some(aux) = get_num_in_pos(&mut matrix[y], x.wrapping_add(1)) {
            flag = true;
            value *= aux;
       } 
    }

    if flag == false {
        return -1;
    }
    return value;
}

fn sum_surrounding_nums_line(x: usize, y: usize, matrix: &mut Matrix) -> u32 {
    let mut value: u32 = 0;
    //print!("y: {}",y);
    if x != 0 {
        value += _get_number_in_pos(&mut matrix[y], x.wrapping_sub(1));
    }
    if value == 0 && matrix[y][x].is_digit(10) {
        return _get_number_in_pos(&mut matrix[y], x);
    }
    value += _get_number_in_pos(&mut matrix[y], x.wrapping_add(1));
    return value;
}

pub fn mult_of_surrounding_nums(x: usize, y: usize, matrix: &mut Matrix) -> i32 {
    let mut value: i32 = 1;
    //println!("Gear ({},{})", x,y);
    let mut temp: i32;
    if y != 0 {
        temp = mult_surrounding_nums_line(x, y.wrapping_sub(1), matrix);
        if temp != -1{
            value *= temp;
        } 
        //println!();
    }
    temp = mult_surrounding_nums_line(x, y, matrix);
    if temp != -1 {
        value *= temp;
    }
    //println!();
    if y < matrix.len().wrapping_sub(1) {
        temp = mult_surrounding_nums_line(x, y.wrapping_add(1), matrix);
        if temp != -1 {
            value *= temp;
        }
        //println!();
    }
    //println!("Gear value = {}", value);
    return value;
}

pub fn sum_of_surrounding_nums(x: usize, y: usize, matrix: &mut Matrix) -> u32 {
    let mut value: u32 = 0;
    //println!("symbol: ({},{})", x, y);
    if y != 0 {
        value += sum_surrounding_nums_line(x, y.wrapping_sub(1), matrix);
        //println!();
    }
    value += sum_surrounding_nums_line(x, y, matrix);
    //println!();
    if y < matrix.len().wrapping_sub(1) {
        value += sum_surrounding_nums_line(x, y.wrapping_add(1), matrix);
        //println!();
    }
    return value;
}

fn sum_nums_in_line(x: usize, line: &Vec<EnginePart>) -> usize {
    let mut value: usize = 0;
    if line[x].is_digit(10) {
        return 1;
    }
    if x > 0 && line[x.wrapping_sub(1)].is_digit(10) {
        value = value.wrapping_add(1);
    } 
    if x.wrapping_add(1) < line.len() && line[x.wrapping_add(1)].is_digit(10) {
        value = value.wrapping_add(1);
    }
    return value;
}

pub fn sum_surrounding_nums(x: usize, y: usize, matrix: &Vec<Vec<EnginePart>>) -> bool {
    let mut value: usize = 0;
    if y != 0 {
        value += sum_nums_in_line(x, &matrix[y.wrapping_sub(1)]);
    }
    value += sum_nums_in_line(x, &matrix[y]);
    if value > 2 {
        return false
    }
    if y < matrix.len().wrapping_sub(1){
        value += sum_nums_in_line(x, &matrix[y.wrapping_add(1)]);
    }
    return value == 2;
}
