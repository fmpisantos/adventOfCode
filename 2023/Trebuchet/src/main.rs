mod read_from_file;
mod aux_methods;
mod my_types;

use read_from_file::{read_input_as_string, read_input_as_matrix, read_input_as_cards, read_input_for_rules_and_seeds}; 
use aux_methods::{get_first_digit, validate_game, min_rgb, sum_of_surrounding_nums, sum_surrounding_nums, mult_of_surrounding_nums, pow};
use my_types::{EnginePart, Print, NewTrait, SeedMap, Solve};

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

fn day4(test: bool) -> usize{
    let mut value = 0;
    let cards = read_input_as_cards("4", test);
    cards.print();

    for card in cards {
        value += pow(card.get_winnigs(), 2);
    }
    return value;
}

fn day4_1(test: bool) -> usize{
    let cards = read_input_as_cards("4.1", test);
    cards.print();
    let mut value = vec!(1; cards.len().clone());
    for (i,card) in cards.iter().enumerate() {
        let count: usize = card.get_winnigs();
        for n in 1..=count {
            value[i + n] += value[i];
        }
    }
    return value.into_iter().reduce(|acc, c| acc + c).unwrap_or(0);
}

fn day5(test: bool) -> usize {
    if let Ok(seed_map) = read_input_for_rules_and_seeds("5", test) {
        // seed_map.print();
        return seed_map.solve();
    }
    0
}

fn day5_1(test: bool) -> usize {
    if let Ok(mut seed_map) = read_input_for_rules_and_seeds("5.1", test) {       
        // seed_map.unwrap_seed_pairs();
        // seed_map.print();
        return seed_map.solve();
    }
    0
}

fn main() {
    // println!("{}", day1(false));
    // println!("{}", day2(false));
    // println!("{}", day2_1(false));
    // println!("{}", day3(false));
    // println!("{}", day3_1(false));
    // println!("{}", day4(false));
    // println!("{}", day4_1(false));
    // println!("{}", day5(false));
    println!("{}", day5_1(false));
}
