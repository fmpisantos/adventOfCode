use crate::aux_methods::pow;
use std::collections::{HashSet, HashMap};

const SEED: &str= "seed";
const LOCATION: &str= "location";

pub type Matrix = Vec<Vec<EnginePart>>;
pub type Card = Vec<Numbers>;
pub type VecPart = Vec<EnginePart>;

pub struct MapConvertion {
    map: HashMap<usize, usize>,
    rule: Vec<[usize; 3]>,
    to: String
}

impl MapConvertion {
    pub fn new(to: String) -> Self {
        Self {
            map: HashMap::new(),
            rule: Vec::new(),
            to
        }
    }

    fn append_rule(&mut self, dest: usize, origin: usize, size: usize) {
        /*
        (0..size).for_each(|u| {
            self.map.insert(origin.wrapping_add(u), dest.wrapping_add(u));
        });
        */
        self.rule.append(&mut vec!([dest, origin, size]));
    } 

    fn get_dest(&self, origin: usize) -> usize {
        for r in &self.rule {
           if origin >= r[1] && origin < r[1].wrapping_add(r[2]) {
               if r[1] > r[0] {
                    return origin.wrapping_sub(r[1].wrapping_sub(r[0]));
               } else {
                    return origin.wrapping_add(r[0].wrapping_sub(r[1]));
               }
           }
        }
        return origin;
    }
}

impl Print for MapConvertion {
    fn print(&self) {
        print!("{:?}", self.rule);
    }
}

pub struct SeedMap {
    pub seeds: Vec<usize>,
    pub map: HashMap<String, MapConvertion>,
}

impl SeedMap {
    pub fn new(seeds: Vec<usize>, map: HashMap<String, MapConvertion>) -> Self {
        Self {
            seeds,
            map
        }
    }

    pub fn init_seeds(&mut self, line: String) {
        self.seeds = line.split(": ").nth(1).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
    }

    pub fn new_map(&mut self, line: String) -> String {
        let mut types = line.split_whitespace().nth(0).unwrap().split("-");
        let from = types.next().unwrap_or_default().to_string();
        let to = types.nth(1).unwrap_or_default().to_string();
        self.map.insert(from.clone(), MapConvertion::new(to));
        from
    }

    pub fn new_rule(&mut self,line: String, from: &String) {
        let mut rules = line.split_whitespace();
        if let Some(item) = self.map.get_mut(from) {
            item.append_rule(rules.next().unwrap_or("0").parse().unwrap_or(0), rules.next().unwrap_or("0").parse().unwrap_or(0), rules.next().unwrap_or("0").parse().unwrap_or(0));
        } 
    }
}

impl Default for SeedMap {
    fn default() -> Self {
        SeedMap::new(Vec::new(), HashMap::new())
    }
}

pub trait Solve {
    fn solve(&self) -> usize;
}

impl Solve for SeedMap {
    fn solve(&self) -> usize{
        let mut min_location = usize::max_value();
        for seed in &self.seeds {
            println!();
            let mut map = self.map.get(SEED).unwrap(); 
            let mut dest: &String;
            let mut idx = seed.clone();
            let mut found_location = false;
            print!("{} {}", map.to, idx);
            while !found_location {
                idx = map.get_dest(idx);
                dest = &map.to;
                print!(", {} {}", dest, idx);
                found_location = dest == LOCATION;
                if found_location {
                    min_location = min_location.min(idx);
                    break;
                }
                map = self.map.get(dest).unwrap();
            }
        }
        println!();
        return min_location;
    }
}

impl Print for SeedMap {
    fn print(&self) {
        println!("Seeds: {:?}", self.seeds);
        for item in &self.map {
            println!("{}-to-{} map:", item.0, item.1.to);
            println!("origin: {:?}", item.1.map.keys());
            println!("to: {:?}", item.1.map.values());
            for rule in &item.1.rule {
                println!("{:?}", rule);
            }
        }
    }
}

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
