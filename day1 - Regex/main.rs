// use std::borrow::Cow;
use std::env;
use std::io::Error;
use std::fs::read_to_string;
use regex::Regex;
use lazy_static::lazy_static;
// use std::convert::TryInto;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  
        .lines()  
        .map(String::from)  
        .collect() 
}

fn translate(before: &str) -> String {

    lazy_static! {
        static ref TEXTONE_REGEX : Regex = Regex::new(
            r"(one)"
            ).unwrap();
    }
    lazy_static! {
        static ref TEXTTWO_REGEX : Regex = Regex::new(
            r"(two)"
            ).unwrap();
    }
    lazy_static! {
        static ref TEXTTHREE_REGEX : Regex = Regex::new(
            r"(three)"
            ).unwrap();
    }
    lazy_static! {
        static ref TEXTFOUR_REGEX : Regex = Regex::new(
            r"(four)"
            ).unwrap();
    }
    lazy_static! {
        static ref TEXTFIVE_REGEX : Regex = Regex::new(
            r"(five)"
            ).unwrap();
    }
    lazy_static! {
        static ref TEXTSIX_REGEX : Regex = Regex::new(
            r"(six)"
            ).unwrap();
    }
    lazy_static! {
        static ref TEXTSEVEN_REGEX : Regex = Regex::new(
            r"(seven)"
            ).unwrap();
    }
    lazy_static! {
        static ref TEXTEIGHT_REGEX : Regex = Regex::new(
            r"(eight)"
            ).unwrap();
    }
    lazy_static! {
        static ref TEXTNINE_REGEX : Regex = Regex::new(
            r"(nine)"
            ).unwrap();
    }
    
    let mut result: String = TEXTONE_REGEX.replace_all(before, "on1e").to_string();
    result = TEXTTWO_REGEX.replace_all(&result, "t2wo").to_string();
    result = TEXTTHREE_REGEX.replace_all(&result, "thr3ee").to_string();
    result = TEXTFOUR_REGEX.replace_all(&result, "fo4ur").to_string();
    result = TEXTFIVE_REGEX.replace_all(&result, "fi5ve").to_string();
    result = TEXTSIX_REGEX.replace_all(&result, "s6ix").to_string();
    result = TEXTSEVEN_REGEX.replace_all(&result, "se7ven").to_string();
    result = TEXTEIGHT_REGEX.replace_all(&result, "eig8ht").to_string();
    result = TEXTNINE_REGEX.replace_all(&result, "ni9ne").to_string();
    return result;
}

fn filter_text(before: &str) -> String {
    lazy_static! {
        static ref TEXTAZ_REGEX : Regex = Regex::new(
            r"[a-z]"
            ).unwrap();
    }    
    let result = TEXTAZ_REGEX.replace_all(before, "").to_string();
    return result;
}


fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let result = read_lines(path);
    let mut sum: i64 = 0;

    for eachline in result {
        
        let translated_text_to_numbers: String = translate(&eachline).to_string();
        let after = filter_text(&translated_text_to_numbers);
        if after.len() >= 2{
            let numbers = format!("{}{}", after.chars().nth(0).unwrap() , after.chars().nth_back(0).unwrap());
            let number: i64 = numbers.trim().parse().expect("Not a number");
            sum += number;
            println!("{} - {} - {} - {} - {}", sum, eachline, translated_text_to_numbers, after, numbers );
        }
        if after.len() < 2{
            let numbers = format!("{}{}", after.chars().nth(0).unwrap() ,after.chars().nth(0).unwrap());
            let number: i64 = numbers.trim().parse().expect("Not a number");
            sum += number;
            println!("{} - {} - {} - {}", sum, eachline, after, numbers );
        }
        println!("------------------------------");            

    }

    return Ok(());
}
