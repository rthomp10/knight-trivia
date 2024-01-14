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
use rand::seq::SliceRandom;

pub fn print_welcome_message(num_questions: usize) {
    println!("Welcome to the Knight Trivia!");
    println!("{}", "
         *_   _   _   _   _   _ *
 ^       | `_' `-' `_' `-' `_' `|       ^
 |       |    Knight Trivia     |       |
 |  (*)  |_   _   _   _   _   _ |  \\^/  |
 | _<\">_ | `_' `-' `_' `-' `_' `| _(#)_ |
o+o \\ / \\0                      0/ \\ / (=)
 0'\\ ^ /\\/                      \\/\\ ^ /`0
   /_^_\\ |                      | /_^_\\
   || || |                      | || ||
   d|_|b_T______________________T_d|_|b
    ");
    println!("Try to guess the correct answer to {num_questions} questions.");
    println!("Type [q]uit at any time to exit the game. Good luck!");
    println!("");
}

#[derive(Clone)]
pub struct Question {
    pub text: String,
    pub answer: usize,
}

pub fn create_questions(num_questions: usize, max_num: usize) -> Result<Vec<Question>, String> {
    let mut questions = vec![
        Question { text: "How many moons does Venus have?".to_string(), answer: 0 },
        Question { text: "If you subtract the number of eyes of a Cyclops from the number of heads of a Cerberus, what do you get?".to_string(), answer: 1 },
        Question { text: "How many sides does a standard coin have?".to_string(), answer: 2 },
        Question { text: "What is the first odd prime number?".to_string(), answer: 3 },
        Question { text: "How many legs do most mammals have?".to_string(), answer: 4 },
        Question { text: "How many fingers does a human hand typically have?".to_string(), answer: 5 },
        Question { text: "What is the number of strings on a standard guitar?".to_string(), answer: 6 },
        Question { text: "How many continents are there on Earth?".to_string(), answer: 7 },
        Question { text: "How many legs does an octopus have?".to_string(), answer: 8 },
        Question { text: "How many lives are cats often said to have?".to_string(), answer: 9 },
        Question { text: "How many pins are set up at the beginning of a bowling lane?".to_string(), answer: 10 },
        Question { text: "How many players are on a soccer team on the field at one time?".to_string(), answer: 11 },
        Question { text: "What is the typical number of hours in half a day?".to_string(), answer: 12 },
        Question { text: "How many in a baker's dozen?".to_string(), answer: 13 },
        Question { text: "What age is typically celebrated as the start of teenage years?".to_string(), answer: 14 },
        Question { text: "The number of days in a fortnight?".to_string(), answer: 15 },
        Question { text: "The atomic number of sulfur?".to_string(), answer: 16 },
        Question { text: "How many syllables are in a haiku (5+7+5)?".to_string(), answer: 17 },
        Question { text: "Legal voting age in many countries?".to_string(), answer: 18 },
        Question { text: "Number of holes in a standard round of golf?".to_string(), answer: 19 },
        Question { text: "The number of permanent teeth in a typical adult human?".to_string(), answer: 20 },
        Question { text: "The number of dots on a standard six-sided die?".to_string(), answer: 21 },
        Question { text: "The number of players on a football (American) team on the field?".to_string(), answer: 22 },
        Question { text: "The number of chromosomes in a human ovum?".to_string(), answer: 23 },
        Question { text: "The number of hours in a day?".to_string(), answer: 24 },
        Question { text: "The number of years in a quarter-century?".to_string(), answer: 25 },
        Question { text: "The number of letters in the English alphabet?".to_string(), answer: 26 },
        Question { text: "The atomic number of cobalt?".to_string(), answer: 27 },
        Question { text: "The number of days in February in a leap year?".to_string(), answer: 28 },
        Question { text: "The number of states in India?".to_string(), answer: 29 },
        Question { text: "The number of days in April, June, September, and November?".to_string(), answer: 30 },
        Question { text: "The number of flavors in Baskin-Robbins' original slogan?".to_string(), answer: 31 },
        Question { text: "The number of pieces on a chessboard at the start of a game?".to_string(), answer: 32 },
        Question { text: "The age Jesus Christ is traditionally believed to have been at the time of his crucifixion?".to_string(), answer: 33 },
        Question { text: "The number of teeth a person has by late adolescence, excluding wisdom teeth?".to_string(), answer: 34 },
        Question { text: "The critical angle in degrees for water, above which total internal reflection occurs?".to_string(), answer: 35 },
        Question { text: "The number of degrees in the interior angles of a hexagon?".to_string(), answer: 36 },
        Question { text: "The normal human body temperature in Celsius?".to_string(), answer: 37 },
        Question { text: "The number of slots in European Roulette?".to_string(), answer: 38 },
        Question { text: "The number of steps in Alfred Hitchcock's film 'The 39 Steps'?".to_string(), answer: 39 },
        Question { text: "The number of weeks in a typical pregnancy?".to_string(), answer: 40 },
        Question { text: "The number of days in the longest possible February (in a leap year)?".to_string(), answer: 41 },
        Question { text: "The answer to life, the universe, and everything, according to 'The Hitchhiker's Guide to the Galaxy'?".to_string(), answer: 42 },
        Question { text: "The normal human chromosome count?".to_string(), answer: 43 },
        Question { text: "The official size of an Olympic marathon in kilometers?".to_string(), answer: 44 },
        Question { text: "The number of U.S. Presidents as of 2023?".to_string(), answer: 45 },
        Question { text: "The number traditionally associated with the years in a sapphire wedding anniversary?".to_string(), answer: 46 },
        Question { text: "The number of human chromosomes that determine the sex of a person? (XX-XY)".to_string(), answer: 47 },
        Question { text: "The atomic number of titanium?".to_string(), answer: 48 },
        Question { text: "The number of contiguous states in the United States?".to_string(), answer: 49 },
        Question { text: "How many stars were on the American flag in 1959?".to_string(), answer: 50 },
        Question { text: "The atomic number of antimony?".to_string(), answer: 51 },
        Question { text: "The number of weeks in a year?".to_string(), answer: 52 },
        Question { text: "The number of cards in a standard deck (excluding jokers)?".to_string(), answer: 53 },
        Question { text: "The number of white keys on a piano?".to_string(), answer: 54 },
        Question { text: "The atomic number of cesium?".to_string(), answer: 55 },
        Question { text: "The number of years in a marriage celebrated as an emerald anniversary?".to_string(), answer: 56 },
        Question { text: "The number of Heinz varieties?".to_string(), answer: 57 },
        Question { text: "The atomic number of iron?".to_string(), answer: 58 },
        Question { text: "The age Madonna was when she released 'Like a Prayer' in 1989?".to_string(), answer: 59 },
        Question { text: "The number of seconds in a minute?".to_string(), answer: 60 },
        Question { text: "The code for international direct dial phone calls to the United Kingdom?".to_string(), answer: 61 },
        Question { text: "The atomic number of samarium?".to_string(), answer: 62 },
        Question { text: "The number of chromosomes found in a chicken?".to_string(), answer: 63 },
        Question { text: "The number of squares on a chessboard?".to_string(), answer: 64 },
        Question { text: "The retirement age for men in the UK as of 2023?".to_string(), answer: 65 },
        Question { text: "The number of parishes in Jamaica?".to_string(), answer: 66 },
        Question { text: "The number of books in the Christian Bible's Old Testament (Protestant version)?".to_string(), answer: 67 },
        Question { text: "The year of the Summer of Love (19XX)?".to_string(), answer: 68 },
        Question { text: "The number of kilometers in a marathon race?".to_string(), answer: 69 },
        Question { text: "The number of years of marriage celebrated at a platinum anniversary?".to_string(), answer: 70 },
        Question { text: "The number of years the Israelites wandered in the wilderness according to the Bible?".to_string(), answer: 71 },
        Question { text: "The number of keys on a standard QWERTY keyboard?".to_string(), answer: 72 },
        Question { text: "The atomic number of tantalum?".to_string(), answer: 73 },
        Question { text: "The number of trombones led the big parade in 'Seventy-Six Trombones'?".to_string(), answer: 74 },
        Question { text: "The diamond wedding anniversary (years of marriage)?".to_string(), answer: 75 },
        Question { text: "The number of trombones in the song 'Seventy-Six Trombones' in 'The Music Man'?".to_string(), answer: 76 },
        Question { text: "The year of the famous UFO incident in Roswell, New Mexico (19XX)?".to_string(), answer: 77 },
        Question { text: "The atomic number of platinum?".to_string(), answer: 78 },
        Question { text: "The number of revolutions per minute of a common vinyl single?".to_string(), answer: 79 },
        Question { text: "The number of years in eight decades?".to_string(), answer: 80 },
        Question { text: "The number of provinces in Canada?".to_string(), answer: 81 },
        Question { text: "The atomic number of lead?".to_string(), answer: 82 },
        Question { text: "The number of stories in Jules Verne's 'Around the World in Eighty Days'?".to_string(), answer: 83 },
        Question { text: "The year George Orwell wrote '1984' minus 1900?".to_string(), answer: 84 },
        Question { text: "The atomic number of polonium?".to_string(), answer: 85 },
        Question { text: "The year Halley's Comet last appeared in the inner parts of the solar system (19XX)?".to_string(), answer: 86 },
        Question { text: "The number of constellations in the sky as recognized by the International Astronomical Union?".to_string(), answer: 87 },
        Question { text: "The number of keys on a standard piano?".to_string(), answer: 88 },
        Question { text: "The atomic number of actinium?".to_string(), answer: 89 },
        Question { text: "The angle in degrees of a right angle?".to_string(), answer: 90 },
        Question { text: "The number of degrees in the sum of the angles of a triangle?".to_string(), answer: 91 },
        Question { text: "The number of poems in the book 'Sonnets from the Portuguese' by Elizabeth Barrett Browning?".to_string(), answer: 92 },
        Question { text: "The atomic number of uranium?".to_string(), answer: 93 },
        Question { text: "The number of counties in the Republic of Ireland?".to_string(), answer: 94 },
        Question { text: "The year the World Wide Web was invented by Tim Berners-Lee (19XX)?".to_string(), answer: 95 },
        Question { text: "The year the United States celebrated its bicentennial anniversary (19XX)?".to_string(), answer: 96 },
        Question { text: "The number of years it took to complete the construction of the Hoover Dam (19XX-19XX)?".to_string(), answer: 97 },
        Question { text: "The temperature in Fahrenheit at which water boils under standard atmospheric pressure?".to_string(), answer: 98 },
        Question { text: "The highest number on a standard BINGO card?".to_string(), answer: 99 },
        Question { text: "The number of tiles in a standard Scrabble game?".to_string(), answer: 100 },
        Question { text: "How many problems does M&M have?".to_string(), answer: 101 },
    ];

    let max_num = std::cmp::min(max_num, questions.len());

    let mut rng = rand::thread_rng();
    questions[..max_num].shuffle(&mut rng);

    if num_questions > max_num {
        return Err(format!(
            "Requested number of questions ({}) exceeds the available questions ({})",
            num_questions,
            max_num
        ));
    }

    return Ok(questions.into_iter().take(num_questions).collect());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_questions_non_empty() {
        let num_questions = 10;
        let max_num = 100;
        let questions = create_questions(num_questions, max_num).unwrap();
        for question in &questions {
            assert!(!question.text.is_empty());
        }
    }

    #[test]
    fn test_questions_unique_answers() {
        let num_questions = 10;
        let max_num = 100;
        let questions = create_questions(num_questions, max_num).unwrap();
        let answers: HashSet<usize> = questions.iter().map(|q| q.answer).collect();
        assert_eq!(answers.len(), questions.len());
    }

    #[test]
    fn test_questions_correct_range() {
        let num_questions = 10;
        let max_num = 50;
        let questions = create_questions(num_questions, max_num).unwrap();
        for question in &questions {
            assert!(question.answer <= max_num);
        }
    }

    #[test]
    fn test_request_too_many_questions() {
        let num_questions = 1000;
        let max_num = 100;
        let result = create_questions(num_questions, max_num);
        assert!(result.is_err());
    }

    #[test]
    fn test_request_zero_questions() {
        let num_questions = 0;
        let max_num = 100;
        let questions = create_questions(num_questions, max_num).unwrap();
        assert_eq!(questions.len(), 0);
    }

    #[test]
    fn test_request_exact_number_of_questions() {
        let num_questions = 10;
        let max_num = 100;
        let questions = create_questions(num_questions, max_num).unwrap();
        assert_eq!(questions.len(), num_questions);
    }
}
