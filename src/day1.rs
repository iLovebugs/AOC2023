use std::convert::TryFrom;
use crate::util::*;


pub fn run_p1(debug : bool) {

    // Input
    let lines : Vec<String> = read_lines("C:/Users/ndmor/aoc2023/src/input/day1");

    // Running sum
    let mut sum : u32 = 0;

    for line in lines {
    
        if debug {println!("The input line is: {}", line);}

        // Filter for digits
        let all_numbers = filter_for_digits(&line);
        
       if debug {println!("Found numbers: {}", all_numbers);}

        // Select first and last number
        let first : &str = &all_numbers[0..1];
        let last : &str = &all_numbers[all_numbers.len()-1..];
        if debug {println!("First number is {}", first);}
        if debug {println!("Last number is {}", last);}
        
        // Create full number as string
        let mut number : String = String::new();
        number = number + last + first;

        // Convert number string to integer
        let number_uint = string_to_uint(&number);
        if debug {println!("Converted to an integer: {}", number_uint)};

        // Create a running sum
        sum += number_uint;
    }

    println!("The final sum is: {}", sum)

}

pub fn run_p2(debug : bool){
    
    // Input
    let lines : Vec<String> = read_lines("C:/Users/ndmor/aoc2023/src/input/day1");

    // Walker
    let mut buffer          : String    = String::with_capacity(5);
    let one_two_six         : [&str; 3] = ["one", "two", "six"];
    let four_five_nine      : [&str; 3] = ["four", "five", "nine"];
    let three_seven_eight   : [&str; 3] = ["three", "seven", "eight"];
    let mut sum             : u32       = 0;
    let mut number          : String    = String::new();
    let mut found3          : bool      = false;  
    let mut found4          : bool      = false;
    let mut found5          : bool      = false;

    for line in lines {
        
        number = String::new();
        found3 = false;
        found4 = false;
        found5 = false;
        buffer.clear();

        for c in line.chars(){
     
            // Push buffer
            buffer.push(c);

            if debug {println!("Buffer is {}", buffer);}
            
            if buffer.len() >= 3 && !found3 {
                for digit in one_two_six{
                    if &buffer[0..3] == digit {
                        number.push_str(&text_to_char_num(digit));
                        found3 = true;
                    }
                }
            }
            if buffer.len() >= 4 && !found4 {
                for digit in four_five_nine{
                    if &buffer[0..4] == digit {
                        number.push_str(&text_to_char_num(digit));
                        found4 = true;
                    }
                }
            }
            if buffer.len() >= 5 && !found5 {
                for digit in three_seven_eight{
                    if &buffer[0..5] == digit {
                        number.push_str(&text_to_char_num(digit));
                        found5 = true;
                    }
                }
                
                // The buffer is full, pop check if pop was digit
                let num : char = buffer.remove(0);
                if num.is_digit(10){
                    number.push(num);
                }

                if debug {println!("Buffer is {}", buffer);}

                found3 = false;
                found4 = false;
                found5 = false;
            }

        }

        // Empty buffer
        while !buffer.is_empty() {

            if buffer.len() >= 3 {
                for digit in one_two_six{
                    if &buffer[0..3] == digit {
                        number.push_str(&text_to_char_num(digit));
                    }
                }
            }
            if buffer.len() >= 4 {
                for digit in four_five_nine{
                    if &buffer[0..4] == digit {
                        number.push_str(&text_to_char_num(digit));
    
                    }
                }
            }
            if buffer.len() >= 5 {
                for digit in three_seven_eight{
                    if &buffer[0..5] == digit {
                        number.push_str(&text_to_char_num(digit));
                    }
                }
            
            }


            let num : char = buffer.remove(0);
            if num.is_digit(10){
                number.push(num);
            }        
        }

        // Select first and last number
        let first : &str = &number[0..1];
        let last : &str = &number[number.len()-1..];
        if debug {println!("First number is {}", first);}
        if debug {println!("Last number is {}", last);}
        
        // Create full number as string
        let mut number : String = String::new();
        number = number + last + first;

        // Convert number string to integer
        let number_uint = string_to_uint(&number);
        if debug {println!("Converted to an integer: {}", number_uint)};

        // Create a running sum
        sum += number_uint;

        
    }

    println!("The final sum is: {}", sum)


}






