/*
 * Knight Trivia: A trivia game for the terminal.
 *
 * Copyright (C) 2024 Ryan Thompson
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::{io, usize};
mod trivia;

fn main() {
    let num_questions = 5;
    let mut questions: Vec<trivia::Question> = Vec::new();

    trivia::print_welcome_message(num_questions);

    match trivia::create_questions(num_questions, usize::MAX) {
        Ok(generated_questions) => {
            questions = generated_questions;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let mut i = 0;
    loop {
        if i > questions.len() - 1 {
            println!("You win!");
            break;
        }

        if let Some(question) = questions.get(i) {
            println!("{}", question.text);
        }

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();

        if guess.contains('q') {
            break;
        }

        match guess.parse::<usize>() {
            Ok(guess) => {
                if let Some(question) = questions.get(i) {
                    print!("\x1B[A\x1B[2K");
                    if question.answer == guess {
                        println!("✔ {guess}");
                        i += 1;
                    } else {
                        println!("✘ {guess}");
                        println!("You're done. The correct answer was {}", question.answer);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
