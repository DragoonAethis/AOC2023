use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;
use std::{env, io};

use aho_corasick::AhoCorasick;

// Shamelessly stolen from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./trebuchet <filename>");
        exit(1);
    }

    let input_filename = &args[1];
    dbg!(input_filename);

    #[rustfmt::skip]
    let digit_words = &[
        "0", "zero",
        "1", "one",
        "2", "two",
        "3", "three",
        "4", "four",
        "5", "five",
        "6", "six",
        "7", "seven",
        "8", "eight",
        "9", "nine",
    ];

    // Build a finder matching any 0-9, or one of the "digit words".
    // Regex cannot find overlapping matches like "twone", so use
    // the underlying matching engine directly, which can.
    // (Yeah, that's from that SO question. You've been there too
    // if you tried to solve it with Regex first.)
    let ac = AhoCorasick::new(digit_words).unwrap();

    if let Ok(lines) = read_lines(input_filename) {
        let mut accumulator: u32 = 0;

        for line in lines {
            if let Ok(data) = line {
                let mut first_number: u32 = u32::MAX;
                let mut last_number: u32 = 0;

                println!("> {}", &data);
                for mat in ac.find_overlapping_iter(data.as_str()) {
                    // Our pattern list contains the digits/words in
                    // the same order as their values, but the value
                    // is multiplied by two, and words have +1 because
                    // they're after the digits.
                    //
                    // So "0" => 0, "zero" => 1, "1" => 2, "one" => 3
                    // and so on. If we divide the pattern ID by 2, we
                    // get the final value without having to parse or
                    // special case the string lookup.

                    let value: u32 = mat.pattern().as_u32() >> 1;
                    println!(
                        "Match: {} @ {}..{} ({})",
                        value,
                        mat.start(),
                        mat.end(),
                        digit_words[mat.pattern().as_usize()]
                    );

                    if first_number == u32::MAX {
                        first_number = value;
                    }

                    last_number = value;
                }

                if first_number != u32::MAX {
                    let result = first_number * 10 + last_number;
                    accumulator += result;
                    println!("Line: {} ({} total)", result, accumulator);
                }
            }
        }

        println!("{}", accumulator);
    }
}
