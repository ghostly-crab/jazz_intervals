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
            "What {} is {} {} {} from {}? {}",
            "note".yellow(),
            if interval == Interval::Octave { "an" } else { "a" },
            interval.to_string().to_lowercase(),
            direction.to_string().to_lowercase(),
            starting_note.to_string().bold().purple(),
            "(or Q to quit)".truecolor(0x77, 0x88, 0x99)
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
                            println!(
                                "{}: Please enter a valid {} like {}, {}, or {}! {}",
                                "Invalid input".bold().red(),
                                "note".yellow(),
                                "D".bold().purple(),
                                "Db".bold().purple(),
                                "D#".bold().purple(),
                                "(or Q to quit)".truecolor(0x77, 0x88, 0x99)
                            );
                            continue;
                        }
                    }
                }
            };
        };

        if guess == resulting_note {
            println!(
                "{}!",
                "Correct".on_green()
            );
        } else {
            println!(
                "{}, the correct answer was {}! {}{}{}",
                "Incorrect".on_red(),
                resulting_note.to_string().bold().purple(),
                "(don't sweat it, wrong answers ".truecolor(0x77, 0x88, 0x99),
                "burn better".italic().cyan(),
                " in the brain)".truecolor(0x77, 0x88, 0x99)
            );
        }
    }
}
