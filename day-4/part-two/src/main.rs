fn draw_scratch_card(line: &str) -> usize {
    let mut words = line.split_whitespace();

    if words.next().unwrap() != "Card" {
        println!("Error: Invalid token");
        std::process::exit(1);
    }

    let _card_number = {
        let word = words.next().unwrap();

        if &word[word.len() - 1..] != ":" {
            println!("Error: Invalid token");
            std::process::exit(1);
        }

        match word[..word.len() - 1].parse::<u32>() {
            Ok(number) => number,
            Err(err) => {
                println!("Error: Invalid number '{}': {}", word, err);
                std::process::exit(1);
            }
        }
    };

    let mut lucky_numbers = Vec::<u32>::new();

    while let Some(word) = words.next() {
        if word == "|" {
            break;
        }

        match word.parse::<u32>() {
            Ok(number) => lucky_numbers.push(number),
            Err(err) => {
                panic!("Error: Invalid number '{}': {}", word, err);
            }
        };
    }

    let mut drawn_numbers = Vec::<u32>::new();

    while let Some(word) = words.next() {
        match word.parse::<u32>() {
            Ok(number) => drawn_numbers.push(number),
            Err(err) => {
                panic!("Error: Invalid number '{}': {}", word, err);
            }
        };
    }

    let mut matched_numbers = Vec::<u32>::new();

    for lucky_number in &lucky_numbers {
        if drawn_numbers.contains(&lucky_number) {
            matched_numbers.push(*lucky_number);
        }
    }

    // println!("Card {}: {:?} | {:?} | {:?}", _card_number, lucky_numbers, drawn_numbers, matched_numbers);

    matched_numbers.len()
}

fn count_recursively(scratch_cards_results: &Vec<usize>, index: usize) -> usize {
    let mut scratch_cards = match scratch_cards_results.get(index).cloned() {
        Some(scratch_cards) => scratch_cards,
        None => return 0,
    };

    let mut index = index + 1;
    let final_index = scratch_cards + index;

    while index < final_index {
        scratch_cards += count_recursively(scratch_cards_results, index);
        index += 1;
    }

    scratch_cards
}

fn main() {
    if std::env::args().len() != 2 {
        println!("Usage: {} <filename>", std::env::args().nth(0).unwrap());
        std::process::exit(1);
    }

    let filename = std::env::args().nth(1).unwrap();

    let file_content = match std::fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(err) => {
            println!("Error: {}: {}", filename, err);
            std::process::exit(1);
        }
    };

    let lines = file_content.lines();

    let scratch_cards_results: Vec<usize> = lines.map(|line| draw_scratch_card(line)).collect();

    let mut total_scratch_cards = scratch_cards_results.len();

    for index in 0..scratch_cards_results.len() {
        total_scratch_cards += count_recursively(&scratch_cards_results, index);
    }

    println!("Total scratch_cards: {}", total_scratch_cards);
}
