use std::io;

struct PolindromeInput {
    value: String
}

/**
* How to check Polindrome 
* 1. Remove non-alphanumeric characters
* 2. Convert to lowercase
* 3. Here we use common pattern (from what I learned through searching and prompting)
* - Instead of reversing the string, we use comparison between left and right
* - where for each iteration we compare the current the words by character from left and right
* - if they are not equal, we return false
* - if they are equal, we add one to left and subtract one from right
* - thus at the end, if we finish the iteration without finding any mismatch, we return true
* 
* tbh this concept is fascinating, need to grind more into software engineering things.
*/

fn is_polindrome(input: &PolindromeInput) -> bool {
    let cleaned: String = input.value
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    let mut left = 0;
    let mut right = cleaned.len() - 1;

    while left < right {
        if cleaned.chars().nth(left) != cleaned.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

fn main() {
    let mut args = String::new();
    println!("type random things:");

    while args.is_empty() || args.trim().is_empty() {
        io::stdin()
        .read_line(&mut args)
        .expect("you must type anything");
    }

    let input = PolindromeInput { value: args };
    let result = is_polindrome(&input);

    if result {
        println!("gotcha, {} is a palindrome", input.value);
    } else {
        println!("{} is not a palindrome", input.value);
    }
}