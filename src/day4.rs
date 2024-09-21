use std::{clone, ops::Index};

use crate::util::*;

pub fn create_deck (debug : bool) -> Vec<Card> {

    let mut card_scores : Vec<(String, u32)> = Vec::new();
    let lines : Vec<String> = read_lines("C:/Users/ndmor/aoc2023/src/input/day4");
    let mut cards : Vec<Card> = Vec::new();

    for line in lines {

        let card_draws_pool : Vec<&str> = line.split(":").collect();
        let card: &&str = card_draws_pool.get(0).expect("The line is malformed, expected Day x : draw | pool");
        let draws_pool : Vec<&str> = card_draws_pool.get(1).expect("The line is malformed, expected Day x : draw | pool").split("|").collect();
        let draws : Vec<&str> = draws_pool.get(0).expect("Draw | pool was expected").split_whitespace().collect();
        let pool  : Vec<&str> = draws_pool.get(1).expect("Draw | pool was expected").split_whitespace().collect();
                
        let mut tmp_draw : Vec<String> = Vec::new();

        for draw in draws{
            tmp_draw.push(draw.to_string());
        }

        let mut tmp_pool : Vec<String> = Vec::new();

        for p in pool{
            tmp_pool.push(p.to_string());
        }

        // -1 to to set position to start from 0
        cards.push(Card::new(string_to_uint(card) - 1, tmp_draw, tmp_pool))

    }

    return cards
}

pub fn run_p1 (debug : bool) {

    let cards = create_deck(debug);
    let mut total_score : u32 = 0;

    // Count matches
    for card in cards {
        total_score += card.score_card();
    }

    println!("The total score is: {}", total_score)
    
}

pub fn run_p2 (debug : bool) {

    let cards = create_deck(debug);
    let mut counters  = vec![1; 199]; // Known input size
    
    // Loop through cards, copy matches
    for card in cards {        
       let matches =  card.return_matches();

        if debug {println!("Card {} has {} matches", card.position, matches);}

       let mut i = 1;
       while i <= matches {
        counters[card.position as usize + i] += counters[card.position as usize];
        i +=1 ;
       }             
    }

    if debug {println!("{:?}", counters)};

    // Get final score
    let mut total : u32 = 0;
    for counter in counters {
        total += counter;
    } 

    println!("Total:{}", total)
}


struct Card {
    position : u32,
    draw : Vec<String>,
    pool : Vec<String>,
}

impl Card {

    fn new (position : u32, draw : Vec<String>, pool : Vec<String>) -> Card {

        Card{position : position, draw : draw, pool : pool}
    }  

    fn print_card (self) {
        println!("This is card has position: {}", self.position);
        println!("The draw is: {:?}", self.draw);
        println!("The pool is: {:?}", self.pool);
    }

    fn score_card (self) -> u32 {
        let mut matches : u32 = 0;
        for draw in &self.draw {
            if self.pool.contains(draw) {              
                matches += 1;    
            }
        }

        if matches == 0 {
            return 0
        }
        else {
            return 2_u32.pow(matches - 1)
        }

    }

    fn return_matches(&self) -> usize {
        let mut temp_matches : Vec<u32> = Vec::new();
        for draw in &self.draw {
            if self.pool.contains(draw) {              
                temp_matches.push(string_to_uint(draw));
            }
         }

         return temp_matches.len()
    }
}


