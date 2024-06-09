use std::io::{self, BufRead};
use regex::Regex;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const DELIMITER_2: &str = r"[!\?。！？…]";
const CLOSING_QUOTATION: &str = r"[\)）」』】］〕〉》\]]";
const ALPHANUMERICS: &str = r"[\d０-９a-zA-Zａ-ｚＡ-Ｚ]";

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let delimiter_re = Regex::new(DELIMITER_2).unwrap();
    let closing_quotation_re = Regex::new(CLOSING_QUOTATION).unwrap();
    let alphanumerics_re = Regex::new(ALPHANUMERICS).unwrap();

    let (sender, receiver) = channel();

    let thread_count = 4; // Adjust based on your system
    let mut handles = Vec::new();

    // Reading input in the main thread
    let mut input = String::new();
    for line in handle.lines() {
        let line = line.unwrap();
        input.push_str(&line);
        input.push('\n');
    }

    // Split input into chunks
    let chunk_size = (input.len() / thread_count) + 1;
    let input_chunks: Vec<String> = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect();

    for (index, chunk) in input_chunks.into_iter().enumerate() {
        let sender = sender.clone();
        let delimiter_re = delimiter_re.clone();
        let closing_quotation_re = closing_quotation_re.clone();
        let alphanumerics_re = alphanumerics_re.clone();

        let handle = thread::spawn(move || {
            process_chunk(chunk, index, sender, &delimiter_re, &closing_quotation_re, &alphanumerics_re);
        });
        handles.push(handle);
    }

    drop(sender);

    let mut results = Vec::new();
    for (index, sentences) in receiver {
        results.push((index, sentences));
    }

    results.sort_by_key(|&(index, _)| index);

    for (_, sentences) in results {
        for sentence in sentences {
            println!("{}", sentence);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn process_chunk(chunk: String, index: usize, sender: Sender<(usize, Vec<String>)>, delimiter_re: &Regex, closing_quotation_re: &Regex, alphanumerics_re: &Regex) {
    let mut local_sentence = String::new();
    let mut sentences = Vec::new();

    let mut char_iter = chunk.chars().peekable();

    while let Some(c) = char_iter.next() {
        local_sentence.push(c);

        if delimiter_re.is_match(&c.to_string()) {
            while let Some(&next_char) = char_iter.peek() {
                if delimiter_re.is_match(&next_char.to_string()) {
                    local_sentence.push(next_char);
                    char_iter.next();
                } else {
                    break;
                }
            }

            if let Some(last_char) = local_sentence.chars().rev().nth(1) {
                if alphanumerics_re.is_match(&last_char.to_string()) {
                    continue;
                }
                if closing_quotation_re.is_match(&last_char.to_string()) {
                    continue;
                }
            }

            if !local_sentence.trim().is_empty() {
                sentences.push(local_sentence.clone());
            }
            local_sentence.clear();
        }
    }

    if !local_sentence.trim().is_empty() {
        sentences.push(local_sentence);
    }

    sender.send((index, sentences)).unwrap();
}
