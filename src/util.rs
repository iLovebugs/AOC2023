use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


// Filters string for digits
pub fn filter_for_digits(input: &str) -> String {
    let mut output : String = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            output.push(c);
        }
    }

    output

}

// Filters digits from string
pub fn filter_digits(input: &str) -> String {
    let mut output : String = String::new();

    for c in input.chars() {
        if !c.is_digit(10) {
            output.push(c);
        }
    }

    output
}

// Converts a string to a unsigned integer
pub fn string_to_uint(input: &str) -> u32 {
    let mut result : u32 = 0;
    let mut pos    : u32 = 0;

    for c in input.chars() {
        match c.to_digit(10_u32) {
            Some(number_10) => {
                result = result.saturating_add(number_10.saturating_mul(10_u32.saturating_pow(pos)));
                pos += 1;
            },
            _ => (),
        }
    }

    result
}

pub fn text_to_char_num (input : &str) -> String {
    let output : String;
    match input {
        "one"   => output = String::from("1"),
        "two"   => output = String::from("2"),
        "three" => output = String::from("3"),
        "four"  => output = String::from("4"),
        "five"  => output = String::from("5"),
        "six"   => output = String::from("6"),
        "seven" => output = String::from("7"),
        "eight" => output = String::from("8"),
        "nine"  => output = String::from("9"),
        _       => output = String::from("0"),
    }

    output
}
