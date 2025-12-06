use std::collections::HashMap;

pub fn string_cleaner(input: &String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

pub fn is_palindrome(input: &String) -> bool {
    let mut left = 0;
    let mut right = input.len() - 1;

    while left < right {
        if input.chars().nth(left) != input.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

pub fn longest_palindrome(input: &String) -> i32 {
    let mut input_map: HashMap<String, i32> = HashMap::new();
    let mut result: i32 = 0;
    let mut odd = false;

    for i in input.chars() {
        if input_map.contains_key(&i.to_string()) {
            input_map.insert(i.to_string(), input_map.get(&i.to_string()).unwrap() + 1);
        } else {
            input_map.insert(i.to_string(), 1);
        }
    }

    for (_, val) in input_map.iter() {
        if val % 2 == 0 {
            result += val;
        } else {
            result += val - 1;
            odd = true;
        }
    }

    if odd {
        result += 1;
    }

    result
}