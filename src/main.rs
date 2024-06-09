use regex::Regex;
use std::io::{self, BufRead};

const DELIMITER: &str = r"[\.!\?．。！？…]";
const CLOSING_QUOTATION: &str = r"[\)）」』】］〕〉》\]]";
const ALPHANUMERICS: &str = r"[\d０-９a-zA-Zａ-ｚＡ-Ｚ]";

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let delimiter_re = Regex::new(DELIMITER).unwrap();
    let closing_quotation_re = Regex::new(CLOSING_QUOTATION).unwrap();
    let alphanumerics_re = Regex::new(ALPHANUMERICS).unwrap();

    let mut local_sentence = String::new();
    let mut char_buffer = Vec::new(); // Buffer to store the last three characters

    for line in handle.lines() {
        let line = line.unwrap();
        let mut char_iter = line.chars().peekable();

        while let Some(c) = char_iter.next() {
            if char_buffer.len() >= 2 {
                char_buffer.remove(0); // Maintain a buffer of last 3 characters
            }
            char_buffer.push(c);

            local_sentence.push(c);

            if delimiter_re.is_match(&c.to_string()) {
                let z = char_buffer.get(0);
                let y = char_buffer.get(1);
                let x = char_iter.peek();

                // Determine whether to split or not
                let should_split = match (z, y, x) {
                    // Example: "...|z |y |x |..."
                    // Delimiter followed by another delimiter
                    (Some(&z), Some(&y), Some(&x)) => {
                        // Repeated delimiter, so skip until next iteration
                        if delimiter_re.is_match(&x.to_string()) {
                            false
                        } else if y == '。' {
                            true
                        // Example: "...|る|。|）|と言った"
                        // Delimiter followed by closing quotation mark
                        } else if closing_quotation_re.is_match(&x.to_string()) {
                            false
                        // Example: "mpl|e |. |c |om/"
                        // Never split between alphanumeric characters unless the delimiter is "。"
                        } else if alphanumerics_re.is_match(&z.to_string())
                            && alphanumerics_re.is_match(&x.to_string())
                            && y != '。'
                        {
                            false
                        } else {
                            true
                        }
                    }
                    // Handle the case where `x` is None (end of line)
                    (Some(&_z), Some(&y), None) => {
                        if y == '。' {
                            true
                        } else {
                            false
                        }
                    }
                    _ => true,
                };

                if should_split {
                    println!("{}", local_sentence.trim());
                    local_sentence.clear();
                }
            }
        }

        // Treat newline as a sentence boundary
        if !local_sentence.trim().is_empty() {
            println!("{}", local_sentence.trim());
            local_sentence.clear();
        }
    }

    // Print any remaining sentence
    if !local_sentence.trim().is_empty() {
        println!("{}", local_sentence.trim());
    }
}
