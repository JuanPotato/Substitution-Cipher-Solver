use std::io::BufRead;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut words: Vec<String> = args[1..].iter().map(|s| s.to_string()).collect();

    if words.len() == 0 {
        eprintln!("No words provided!");
        println!("Put scrambled text as arguments and input dictionary as text input");
        std::process::exit(1);
    }

    let mut word_maps: Vec<Vec<Vec<usize>>> = words.iter().map(|w| make_word_map(w)).collect();
    let mut word_matches: Vec<Vec<String>> = vec![Vec::new(); words.len()];

    for (word, map) in words.iter().zip(&word_maps) {
        println!("{}: {:?}", word, map);
    }

    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    let mut eof = false;

    while !eof {
        match handle.read_line(&mut line) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                line.pop(); // remove newline

                // Find a way to only do this when the length is not the same
                let line_map = make_word_map(&line);

                'words: for (i, (word, word_map)) in words.iter().zip(&word_maps).enumerate() {
                    if line.len() != word.len() {
                        continue 'words;
                    }

                    if line_map == *word_map {
                        println!("{}: Got a match: {}", word, line);
                        word_matches[i].push(line.clone());
                    }
                }
                
                line.clear();

            }
            Err(_error) => { panic!("something went wrong"); }
        }
    }

    while word_matches.len() > 1 {
        // merge the last two words
        let word_a = words.pop().unwrap();
        let word_b = words.pop().unwrap();
        word_maps.pop();
        word_maps.pop();

        let new_word = word_b + " " + &word_a;
        let new_map = make_word_map(&new_word);
        let mut new_matches = Vec::new();

        let matches_a = word_matches.pop().unwrap();
        let matches_b = word_matches.pop().unwrap();

        for match_a in &matches_a {
            for match_b in &matches_b {
                let new_match = format!("{} {}", match_b, match_a);

                let match_map = make_word_map(&new_match);
                if match_map == new_map {
                    println!("{}: Got a match: {}", &new_word, &new_match);
                    new_matches.push(new_match);
                }
            }
        }

        words.push(new_word);
        word_maps.push(new_map);
        word_matches.push(new_matches);
    }
}

fn make_word_map(word: &str) -> Vec<Vec<usize>> {
    let mut word_map: Vec<Vec<usize>> = Vec::with_capacity(26);
    let mut char_order = Vec::with_capacity(26);
    
    'outer: for (i, c) in word.char_indices() {
        for (ordered_index, ordered_char) in char_order.iter().enumerate() {
            if c == *ordered_char {
                word_map[ordered_index].push(i);
                continue 'outer;
            }
        }

        char_order.push(c);
        word_map.push(Vec::with_capacity(8));
        word_map.last_mut().unwrap().push(i);
    }
    
    return word_map;
}
