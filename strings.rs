// Exercise 1
#[allow(dead_code)]
fn exercise1(color: &str) -> String {
    // Return a string that is the same as color
    color.to_string()
}

// Exercise 2
// Fix all errors without adding newline
fn exercise2() -> String {
    // Declare s as a mutable variable
    let mut s = String::from("hello");
    s.push(',');
    // Use push_str to append a string to s
    s.push_str(" world");
    // Use += to append a string to s
    s += "!";
    s
}
// Exercise 3
// Fix errors without removing any line
fn exercise3() -> String {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // Add & before s1 and s2 and add ; at the end
    let s3 = &s1 + &s2;
    s3
}

// Exercise 4
// Reverse a string

fn reverse_string(input: &str) -> String {
    // Initialize an empty string as the result
    let mut result = String::new();
    // Loop through the characters in the input from the end to the start
    for c in input.chars().rev() {
        // Append the character to the result
        result.push(c);
    }
    // Return the result
    result
}


// Exercise 5
// Check if a string is a palindrome
fn is_palindrome(word: &str) -> bool {
    // Convert the word to lowercase and remove any whitespace
    let word = word.to_lowercase().trim();
    // Get the length of the word
    let len = word.len();
    // If the word is empty or has one character, it is a palindrome
    if len <= 1 {
        return true;
    }
    // Get the bytes of the word as a slice
    let bytes = word.as_bytes();
    // Use two indices to compare the first and last characters of the word
    let mut i = 0;
    let mut j = len - 1;
    // Loop until the indices meet in the middle
    while i < j {
        // If the characters don't match, the word is not a palindrome
        if bytes[i] != bytes[j] {
            return false;
        }
        // Increment the left index and decrement the right index
        i += 1;
        j -= 1;
    }
    // If the loop finishes, the word is a palindrome
    return true;
}

// Exercise 6
// Count the occurrences of a character in a string
fn count_char_occurrences(string: &str, ch: char) -> usize {
    // Initialize a counter variable to zero
    let mut count = 0;
    // Convert the target character to lowercase
    let ch = ch.to_lowercase().next().unwrap();
    // Loop over the characters of the string
    for c in string.chars() {
        // Convert the current character to lowercase
        let c = c.to_lowercase().next().unwrap();
        // If the characters match, increment the counter
        if c == ch {
            count += 1;
        }
    }
    // Return the final count
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn exercise1_work() {
        assert_eq!("white".to_string(), exercise1("white"));
    }

    // Test for exercise 2
    #[test]
    fn exercise2_work() {
        assert_eq!("hello, world!".to_string(), exercise2());
    }

    // Test for exercise 3
    #[test]
    fn exercise3_work() {
        assert_eq!("hello, world!".to_string(), exercise3());
    }
    
    // Test for exercise 4
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("world"), "dlrow");
        assert_eq!(reverse_string(""), "");
    }

    // Test for exercise 5
    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("level"), true);
        assert_eq!(is_palindrome("deed"), true);
        assert_eq!(is_palindrome("Rotor"), true);
    }
    // Test for exercise 5
    #[test]
    fn test_non_palindrome() {
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("world"), false);
    }

    // Test for exercise 6

    #[test]
    fn test_count_char_occurrences() {
        assert_eq!(count_char_occurrences("Hello", 'l'), 2);
        assert_eq!(count_char_occurrences("Rust is fun", 'u'), 1);
        assert_eq!(count_char_occurrences("Mississippi", 's'), 4);
    }

}