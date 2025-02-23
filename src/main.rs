// external
use crossterm::{cursor::MoveUp, execute, terminal::{self, Clear}};
use rand::Rng;
use std::{io, process};
use colored::Colorize;

// internal
use jazz_intervals::enums::{Direction, Interval, Note};

fn main() {
    println!("----- Jazz Intervals -----");
    
    let mut rng = rand::thread_rng();
    
    loop {
        let starting_note: Note = rng.gen_range(0..=11).try_into().expect("Range is limited to allowable values, so should be guaranteed to always be valid");
        let interval: Interval = rng.gen_range(0..=11).try_into().expect("Range is limited to allowable values, so should be guaranteed to always be valid");
        let direction: Direction = rng.gen_range(0..=1).try_into().expect("Range is limited to allowable values, so should be guaranteed to always be valid");
        
        let resulting_note = starting_note.from_interval(direction, interval);

        println!(
            "What {} is {} {} {} from {}? (or Q to quit)",
            "note".yellow(),
            if interval == Interval::Octave { "an" } else { "a" },
            interval.to_string().to_lowercase(),
            direction.to_string().to_lowercase(),
            starting_note.to_string().bold().purple(),
        );
        
        let mut guess = String::new();
        
        let mut first_iteration = true;
        let guess: Note = loop {
            guess.clear();

            io::stdin()
               .read_line(&mut guess)
                .expect("Failed to read line");

            let mut guess_chars = guess.chars();
            if let Some(first) = guess_chars.next() {
                guess = first.to_uppercase().chain(guess_chars).collect();
            }

            match guess.trim() {
                "Q" => process::exit(0),
                guess => {
                    match Note::try_from(guess) {
                        Ok(note) => break note,
                        Err(_) => {
                            if first_iteration {
                                first_iteration = false;
                            } else {
                                execute!(
                                    io::stdout(),
                                    MoveUp(1),
                                    Clear(terminal::ClearType::CurrentLine)
                                ).unwrap();
                            }
                            execute!(
                                io::stdout(),
                                MoveUp(1),
                                Clear(terminal::ClearType::CurrentLine),
                            ).unwrap();
                            println!("Invalid input, please enter a valid note like D, Db, or D#! (or Q to quit)");
                            continue;
                        }
                    }
                }
            };
        };

        if guess == resulting_note {
            println!("Correct!");
        } else {
            println!("Incorrect, the correct answer was {resulting_note}! (don't sweat it, wrong answers burn better in the brain)");
        }
    }
}
