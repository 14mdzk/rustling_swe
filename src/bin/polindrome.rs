use std::io;

struct PolindromeInput {
    value: String
}

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