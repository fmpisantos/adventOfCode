use std::{fs::File, io::{Read, self, BufRead}};

type Matrix = Vec<Vec<EnginePart>>;

fn get_path(test: bool) -> String {
    let mut path = String::from("src/inputs/");
    if test {
        path += "test";
    } else {
        path += "day";
    }
    //println!("Executing: {}", path);
    return path;
}

fn read_input_as_matrix(day: &str, test: bool) -> Matrix {
    return read_file_as_matrix(&format!("{}{}", get_path(test), day)).unwrap();    
}

fn read_file_as_matrix(file_path: &String) -> io::Result<Matrix> {
    //println!("{}", file_path);
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

fn read_input_as_string(day: &str, test: bool) -> String {
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
                    //println!("readToString Failled");
                    return String::new();
                }
            }
        }
        Err(_) => {
            //println!("Cannot find file @{:?}", filename);
            return String::new();
        }
    }
}

fn get_first_digit(line: &mut Vec<EnginePart>, reversed: bool, consider_string: bool) -> u32 {
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

fn get_first_digits(line: &mut Vec<EnginePart>, reversed: bool, multiple: bool, consider_string: bool) -> u32 {
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

fn day1(test: bool) -> u32{
    let input: String = read_input_as_string("1.1", test);
    let mut value = 0;
    for line in input.lines() {
        let mut chars: Vec<EnginePart> = line.chars().map(|ch| EnginePart::new(ch)).collect();
        value += get_first_digit(&mut chars, false, true) * 10;
        value += get_first_digit(&mut chars, true, true);
    }
    return value;
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

fn validate_game(game: Vec<&str>, rgb: &[u32;3]) -> u32 {
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


fn min_rgb(game: Vec<&str>) -> [u32;3] {
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

fn day2(test: bool) -> u32{
    // 12 red cubes, 13 green cubes, and 14 blue cubes.
    let rgb = [12,13,14];
    let input: String = read_input_as_string("2", test);
    let mut value = 0;
    for line in input.lines() {
        //println!("{}", line);
        let to_add = validate_game(line.split(": ").collect(), &rgb);
        //println!("{}", to_add);
        value += to_add;
    }
    return value;
}

fn day2_1(test: bool) -> u32 {
    let input: String = read_input_as_string("2.1", test);
    let mut value = 0;
    for line in input.lines() {
        //println!("{}", line);
        let to_add = min_rgb(line.split(": ").collect());
        let sum = to_add.get(0).unwrap_or(&0) * to_add.get(1).unwrap_or(&0) * to_add.get(2).unwrap_or(&0);
        //println!("[{},{},{}] => {}", to_add.get(0).unwrap_or(&0), to_add.get(1).unwrap_or(&0), to_add.get(2).unwrap_or(&0),sum);
        value += sum;
    }
    return value;
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

fn mult_of_surrounding_nums(x: usize, y: usize, matrix: &mut Matrix) -> i32 {
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

fn sum_of_surrounding_nums(x: usize, y: usize, matrix: &mut Matrix) -> u32 {
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

trait Print {
    fn print(&self); 
}

impl Print for Matrix {
    fn print(&self) {
        for line in self {
            for c in line {
                print!("{}", c.ch);
            }
            println!();
        }
    }
}

#[derive(Clone)]
struct EnginePart {
    ch: char,
    calculated: &'static bool
}

impl EnginePart {
    fn get_if_not_seen(&self) -> char {
        // //print!("original: {}", self.ch);
        if *self.calculated {
            //  //print!(" returned: {}", 0);
            return '0';
        }
        // //print!(" returned: {}", self.ch);
        return self.ch;
    }

    fn is_gear(&self) -> bool {
       match self.ch {
            '*' =>true, 
            _ => false
       } 
    }
}

trait IsDigit {
    fn is_digit(&self, base: u32) -> bool; 
    fn to_digit(&mut self, base: u32) -> Option<u32>;
}

trait NewTrait {
    fn new(ch: char) -> Self;
    fn is_symbol(&self) -> bool;
    fn seen(&self) -> bool;
    fn see(&mut self) -> char;
}

impl NewTrait for EnginePart  {
    fn new(ch: char) -> Self {
        EnginePart { ch, calculated: &false }
    }


    fn is_symbol(&self) -> bool{
        return !self.ch.is_alphanumeric() && self.ch != '.';
    }

    fn seen(&self) -> bool {
        return *self.calculated;
    }

    fn see(&mut self) -> char {
        self.calculated = &true;
        self.ch
    }
}

impl IsDigit for EnginePart {
    fn to_digit(&mut self, base: u32) -> Option<u32> {
        self.see();
        //println!("{}", self.ch);
        if !self.calculated && self.is_digit(base) {
            Some(0)
        } else {
            self.ch.to_digit(base)
        }
    }

    fn is_digit(&self, base: u32) -> bool {
        return self.ch.is_digit(base);
    }
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

fn sum_surrounding_nums(x: usize, y: usize, matrix: &Vec<Vec<EnginePart>>) -> bool {
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

fn day3(test: bool) -> u32 {
    let mut value = 0;
    let mut matrix = read_input_as_matrix("3", test);
    matrix.print();
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x].is_symbol(){
                value += sum_of_surrounding_nums(x,y, &mut matrix);
            }
        }
    }
    return value;
}

fn day3_1(test: bool) -> i32 {
    let mut value = 0;
    let mut matrix = read_input_as_matrix("3.1", test);

    matrix.print();
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x].is_gear() && sum_surrounding_nums(x,y, &matrix){
                //println!();
                value += mult_of_surrounding_nums(x,y, &mut matrix);
            }
        }
    }
     return value;
}

fn main() {
    // //println!("{}", day1(false));
    // //println!("{}", day2(false));
    // //println!("{}", day2_1(false));
    // //println!("{}", day3(false));
    println!("{}", day3_1(false));
}
