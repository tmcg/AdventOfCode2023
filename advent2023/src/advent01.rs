use crate::shared;

const DAY_NUMBER: i32 = 1;

fn find_numbers(s: &String, use_words: bool) -> (u32, u32) {
  let mut first: Option<u32> = None; 
  let mut last: Option<u32> = None;
  let mut curr: Option<u32>;

  for i in 0..s.len() {
    curr = None;
    let ss = s.get(i..).unwrap();
    if ss.starts_with('1') || (use_words && ss.starts_with("one")) { curr = Some(1); }
    if ss.starts_with('2') || (use_words && ss.starts_with("two")) { curr = Some(2); }
    if ss.starts_with('3') || (use_words && ss.starts_with("three")) { curr = Some(3); }
    if ss.starts_with('4') || (use_words && ss.starts_with("four")) { curr = Some(4); }
    if ss.starts_with('5') || (use_words && ss.starts_with("five")) { curr = Some(5); }
    if ss.starts_with('6') || (use_words && ss.starts_with("six")) { curr = Some(6); }
    if ss.starts_with('7') || (use_words && ss.starts_with("seven")) { curr = Some(7); }
    if ss.starts_with('8') || (use_words && ss.starts_with("eight")) { curr = Some(8); }
    if ss.starts_with('9') || (use_words && ss.starts_with("nine")) { curr = Some(9); }
    if ss.starts_with('0') || (use_words && ss.starts_with("zero")) { curr = Some(0); }

    if curr.is_some() {
      last = curr;
      if first.is_none() {
        first = curr;
      }
    }
  }

  (first.unwrap_or(0), last.unwrap_or(0))
}

pub fn part1() -> String {
  let lines = shared::input_file_as_lines(DAY_NUMBER);

  lines.iter()
    .map(|s| find_numbers(s, false))
    .map(|t| (t.0 * 10) + t.1)
    .sum::<u32>()
    .to_string()
}

pub fn part2() -> String {
  let lines = shared::input_file_as_lines(DAY_NUMBER);

  lines.iter()
    .map(|s| find_numbers(s, false))
    .map(|t| (t.0 * 10) + t.1)
    .sum::<u32>()
    .to_string()
}

#[cfg(test)]
mod tests {
  #[test]
  fn solve_part1() {
    assert_eq!(super::part1(), "54667");
  }

  #[test]
  fn solve_part2() {
    assert_eq!(super::part2(), "54203");
  }
}
