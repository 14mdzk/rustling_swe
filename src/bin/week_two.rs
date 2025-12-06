use std::io::{self, Write};
use rustling_swe::utils::string::{self, string_cleaner};



fn main() {
    let mut args = String::new();
    println!("
===================================================
=           WEEK TWO - Module System              =
=           LONGEST PALINDROME                    =
===================================================
    ");

    loop {
        print!("type some words: ");
        let _ = io::stdout().flush();

        io::stdin()
        .read_line(&mut args)
        .expect("please type something");
        
        args = string_cleaner(&args);
        if !args.is_empty() || !args.trim().is_empty() {
            break;
        }

        println!("");
        print!("please ");

        args.clear();
    }
    println!("");
    println!("based on {} the longest palindome available are {}", args, string::longest_palindrome(&args));
}