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

fn recursive_draw_scratch_cards(lines: &Vec<&str>, index: usize) -> usize {
    let line = match lines.get(index) {
        Some(line) => line,
        None => return 0,
    };

    let mut scratch_cards = draw_scratch_card(line);

    let mut index = index + 1;
    let end = scratch_cards + index;

    while index < end {
        scratch_cards += recursive_draw_scratch_cards(lines, index);
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

    let lines: Vec<&str> = file_content.lines().collect();

    let mut total_scratch_cards = lines.len();

    for line_index in 0..lines.len() - 1 {
        total_scratch_cards += recursive_draw_scratch_cards(&lines, line_index);
    }

    println!("Total points: {}", total_scratch_cards);
}
