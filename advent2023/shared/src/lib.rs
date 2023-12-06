
pub fn input_as_lines(s: &str) -> Vec<String> {
    s.split("\r\n").map(|x| x.to_owned()).collect::<Vec<_>>()
}

pub fn input_as_ints(s: &str) -> Vec<i64> {
    let input_lines = input_as_lines(s);

    input_lines.iter()
        .map(|s| s.parse::<i64>().expect("Unable to convert line to i64"))
        .collect()
}
