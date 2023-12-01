use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn input_file_as_string(day_number: i32) -> String {
    let read_err = format!("Unable to read lines for day {}", day_number);
    let path = input_file_for_day(day_number);

    read_to_string(path).expect(&read_err)
}

pub fn input_file_as_lines(day_number: i32) -> Vec<String> {
    let open_err = format!("Unable to open input for day {}", day_number);
    let read_err = format!("Unable to read lines for day {}", day_number);

    let path = input_file_for_day(day_number);
    let file = File::open(path).expect(&open_err);

    BufReader::new(file)
        .lines()
        .map(|a| a.expect(&read_err))
        .collect()
}

pub fn input_file_as_ints(day_number: i32) -> Vec<i64> {
    let input_lines = input_file_as_lines(day_number);

    input_lines.iter()
        .map(|s| s.parse::<i64>().expect("Unable to convert line to i64"))
        .collect()
}

fn input_file_for_day(day_number: i32) -> PathBuf {
    let input_file = format!("advent{:02}.txt", day_number);
    let input_path = env::current_dir().expect("Unable to get cwd");

    input_path.join("data").join(input_file)
}
