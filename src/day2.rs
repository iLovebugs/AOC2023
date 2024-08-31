use crate::util::*;

pub fn run_p1 (debug : bool) {

    // Define constants
    // 12 red cubes, 13 green cubes, and 14 blue cubes.
    const RED_CUBES_MAX   : u32 = 12;
    const GREEN_CUBES_MAX : u32 = 13;
    const BLUE_CUBES_MAX  : u32 = 14;
   
    let input : Vec<String> = read_lines("C:/Users/ndmor/aoc2023/src/input/day2");
    let mut game_id_rounds  : Vec<&str>;
    let mut game_id         : String;
    let mut game_id_u32     : u32;
    let mut game            : &str;
    let mut rounds          : Vec<&str>;
    let mut rgb             : Vec<&str>;
    let mut sum             : u32 = 0;
    let mut viable          : bool = true;

    for line in input {

      viable = true;

      game_id_rounds = line.split(":").collect();

      game_id = filter_for_digits(&game_id_rounds[0]);
      game_id_u32 = string_to_uint(&game_id);

      if debug {println!("Game ID: {}", game_id_u32);}

      game  = game_id_rounds[1];

      rounds   = game.split(";").collect();

      for round in rounds {
        
        rgb = round.split(",").collect();

        for color in rgb {
            
            if debug {println!("Found {}", color)}

            let number_str = filter_for_digits(color);
            let number : u32 = string_to_uint(&number_str);

          
            // P1
            if color.contains("red") {
                
                if number > RED_CUBES_MAX { 
                    viable = false; 

                    if debug {println!("Too many reds: {}", number);}
                    
                }

            } else if color.contains("green") {

                if number > GREEN_CUBES_MAX { 
                    viable = false;
                    
                    if debug {println!("Too many greens: {}", number);}
                 }

            } else {

                if number > BLUE_CUBES_MAX { viable = false; 
                    if debug {println!("Too many blues: {}", number);}
                }
            }          
            
        }

      }

        if debug {
            if viable {
                println!("Game ID: {} is viable", game_id_u32);
            }  
        }

        // Sum of all viable game IDs:
        if viable {
            sum += game_id_u32;
        }



    }


    println!("The sum of all viable games IDs: {}", sum);

}

pub fn run_p2(debug : bool) {

    let input : Vec<String> = read_lines("C:/Users/ndmor/aoc2023/src/input/day2");
    let mut game_id_rounds  : Vec<&str>;
    let mut game_id         : String;
    let mut game_id_u32     : u32;
    let mut game            : &str;
    let mut rounds          : Vec<&str>;
    let mut rgb             : Vec<&str>;
    let mut red_max         : u32 = 1;
    let mut green_max       : u32 = 1;
    let mut blue_max        : u32 = 1;
    let mut sum             : u32 = 0;

    for line in input {

      game_id_rounds = line.split(":").collect();

      game_id = filter_for_digits(&game_id_rounds[0]);
      game_id_u32 = string_to_uint(&game_id);

      if debug {println!("Game ID: {}", game_id_u32);}

      game  = game_id_rounds[1];

      rounds   = game.split(";").collect();


      // Reset max, set to one otherwise power multiplication fails
      red_max   = 1;
      green_max = 1;
      blue_max  = 1;


      for round in rounds {
        
        rgb = round.split(",").collect();

        for color in rgb {
            
            if debug {println!("Found {}", color)}

            let number_str = filter_for_digits(color);
            let number : u32 = string_to_uint(&number_str);

          
            // P1
            if color.contains("red") {
                
                if number > red_max {
                    red_max = number;

                    if debug {println!("The red max is : {}", red_max);}
                }

            } else if color.contains("green") {

                if number > green_max {
                    green_max = number;

                    if debug {println!("The red max is : {}", green_max);}
                }

            } else {

                if number > blue_max {
                    blue_max = number;

                    if debug {println!("The red max is : {}", blue_max);}
                }
            }          
            
        }

      }

      sum += red_max * green_max * blue_max;
    
    }

    println!("The sum of game powers are: {}", sum)

}