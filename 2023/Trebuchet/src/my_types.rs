use crate::aux_methods::pow;
use std::collections::HashSet;

pub type Matrix = Vec<Vec<EnginePart>>;
pub type Card = Vec<Numbers>;
pub type VecPart = Vec<EnginePart>;

trait Import {
    fn import(char_vec: Vec<char>) -> Self;
}

impl Print for Card {
    fn print(&self) {
        for card in self {
            card.print();
        }
    }
}

impl Import for VecPart {
    fn import(char_vec: Vec<char>) -> Self {
        char_vec.iter().map(|&c| EnginePart::new(c)).collect()
    }
}

pub struct Numbers {
   pub mine: HashSet<usize>,
   pub winning: HashSet<usize>,
   pub card_nr: usize
}

impl Print for Numbers {
    fn print(&self) {
        print!("Card {}: ", self.card_nr);
        for num in &self.mine {
            print!("{} ", num);
        }
        print!(" | ");
        for num in &self.winning {
            print!("{} ", num);
        }
        println!();
    }
}

impl Numbers {
    pub fn new(mine: HashSet<usize>, winning: HashSet<usize>, card_nr: usize) -> Self {
        Self {
            mine,
            winning,
            card_nr
        }
    }

    pub fn parse_nums(set: &mut HashSet<usize>, nums: &str) {
        let splited = nums.split_whitespace();
        for num in splited {
            set.insert(num.parse().unwrap_or(0));
        }
    }

    pub fn get_winnigs(&self) -> usize {
        let mut result: usize = 0;
        self.print();
        print!("Winning numbers: (");
        if self.mine.len() > self.winning.len() {
            for number in &self.winning {
                if self.mine.contains(number) {
                    if number > &0 {
                        print!(" ,");
                    }
                    print!("{}", number);
                    result = result.wrapping_add(1);
                }
            }
        } else {
            for number in &self.mine{
                if self.winning.contains(number) {
                    if number > &0 {
                        print!(" ,");
                    }
                    print!("{}", number);
                    result = result.wrapping_add(1);
                }
            }
        }
        if result == 0 {
            print!(") -> {} winners = {}", result, result);
            println!();
            return 0;
        }
        print!(") -> {} winners = {}", result, pow(result.saturating_sub(1), 2));
        println!();
        result
    }
}

impl Default for Numbers {
    fn default() -> Self {
        return Self::new(HashSet::new(), HashSet::new(), 0);
    }
}

pub trait Print {
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
pub struct EnginePart {
    pub ch: char,
    pub calculated: &'static bool
}

impl EnginePart {
    pub fn new(ch: char) -> Self { Self { ch, calculated: &false} }

    pub fn get_if_not_seen(&self) -> char {
        // //print!("original: {}", self.ch);
        if *self.calculated {
            //  //print!(" returned: {}", 0);
            return '0';
        }
        // //print!(" returned: {}", self.ch);
        return self.ch;
    }

    pub fn is_gear(&self) -> bool {
       match self.ch {
            '*' =>true, 
            _ => false
       } 
    }
}

pub trait IsDigit {
    fn is_digit(&self, base: u32) -> bool; 
    fn to_digit(&mut self, base: u32) -> Option<u32>;
}

pub trait NewTrait {
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
