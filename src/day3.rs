use std::collections::binary_heap::Iter;

use crate::util::*;

pub fn run_p1 (debug : bool) {

    let numbers             : Vec<Slice>;
    let special_chars       : Vec<Slice>;

    (numbers, special_chars) = create_slices(debug);
    adjacent_sum(&numbers, &special_chars, debug);


}

pub fn run_p2 (debug : bool) {

    let numbers             : Vec<Slice>;
    let special_chars       : Vec<Slice>;
    (numbers, special_chars) = create_slices(debug);
    proper_gears(&numbers, &special_chars, debug);
}


fn adjacent_sum (numbers : &Vec<Slice>, special_chars: &Vec<Slice>, debug : bool) {

    let mut sum : u32 = 0;

    for i  in 0..numbers.len() {

        for j in 0..special_chars.len(){

            let tmp_num = numbers.get(i).unwrap();
            let tmp_special = special_chars.get(j).unwrap();

            if tmp_num.adjacent(tmp_special, debug){

                if debug {println!("The value {} is adjacent to {}", tmp_num.value_str, tmp_special.value_str);}
                sum += string_to_uint(&tmp_num.value_str);
            }        
        }
    }

    println!("The sum is: {}", sum)

}

fn proper_gears (numbers : &Vec<Slice>, special_chars: &Vec<Slice>, debug : bool) {

    let mut adjacent_vect : Vec<u32> = Vec::new();
    let mut gear_sum      : u32 = 0;

    // Go through all special chars
    for i in 0..special_chars.len(){
        
        let tmp_special = special_chars.get(i).unwrap();

        adjacent_vect.clear();
        
        // Is it a gear?
        if tmp_special.value_str == "*" {

            for j in 0..numbers.len(){
                let tmp_number = numbers.get(j).unwrap();

                if tmp_special.adjacent(tmp_number, debug) {
                    adjacent_vect.push(string_to_uint(&tmp_number.value_str));
                }
            }
        }  

        if debug {
            println!("Found gear, it is adjacent to {:?}", adjacent_vect)
        }

        if adjacent_vect.len() == 2 {

            if debug {
                println!("Found proper gear, it is adjacent to {:?}", adjacent_vect)
            }

            gear_sum += adjacent_vect.get(0).unwrap() * adjacent_vect.get(1).unwrap();
        }
    }

    println!("The gear sum is: {}", gear_sum)

}


fn create_slices (debug : bool) -> (Vec<Slice>, Vec<Slice>) {

    let input               : Vec<String> = read_lines("C:/Users/ndmor/aoc2023/src/input/day3");
    let mut y               : u32 = 0;
    let mut x               : u32 = 0;
    
    let mut special_chars   : Vec<Slice> = Vec::new();
    let mut numbers         : Vec<Slice> = Vec::new();
    let mut pending_slice   : bool = false;

    for line in input {

        // Reset x
        x = 0;

        let mut line_iter = line.chars();

        while let Some(c) = line_iter.next() {
            // *$+-#%
            if is_special_char(&c) {
                special_chars.push(Slice::new(x, y, 1, &c.to_string()));
                pending_slice = false;
            }
            // 0-9
            else if c.is_digit(10) {
                if pending_slice {
                    let mut tmp : Slice = numbers.pop().unwrap();
                    tmp.len += 1;
                    tmp.value_str.push(c);
                    numbers.push(tmp);
                }
                else {
                    pending_slice = true;
                    numbers.push(Slice::new(x, y, 1, &c.to_string()))
                }
            }
            // .
            else {
                pending_slice = false;
            }

            x += 1;
        }

        y += 1;
        
    }

    if debug { 
        
        for i in 0..numbers.len() {

            println!("Found number: {:?}", numbers.get(i).unwrap());
        }

        for i in 0..special_chars.len() {

            println!("Found special character: {:?}", special_chars.get(i).unwrap());
        }

    }
    

    return (numbers, special_chars)    

}

#[derive(Debug)]
pub struct Slice {
    x         : u32,
    y         : u32,
    len       : u32,
    value_str : String,
    
}

impl Slice {

    pub fn new (x : u32, y : u32, len : u32, value_str : &str) -> Slice {
        
        Slice {x : x, y : y, len : len, value_str : value_str.to_string()}
    }

    pub fn adjacent(&self, other : &Slice, debug : bool) -> bool {

        let mut x_adjacent : bool = false;
        let mut y_adjacent : bool = false;

        if other.x < self.x {
            if other.x + other.len >= self.x {
                x_adjacent = true;
            }
        }        
       else if other.x > self.x {
            if self.x + self.len >= other.x {
                x_adjacent = true;
            }
       }
       else {
           x_adjacent = true
       }

        if debug && x_adjacent {
            println!("{:?} is x adjacent to {:?}", self, other);
        }

        if self.y > 0 {
            if self.y - 1 == other.y {
                y_adjacent = true;
            }
            else if self.y == other.y {
                y_adjacent = true;
            }
            else if self.y  + 1 == other.y {
                y_adjacent = true;
            }
        }
        else {
            if self.y == other.y {
                y_adjacent = true;
            }
            else if self.y + 1 == other.y {
                y_adjacent = true;
            }
        }

        if debug && y_adjacent {
            println!("{:?} is y adjacent to {:?}", self, other);
        }

        x_adjacent && y_adjacent
    }
}