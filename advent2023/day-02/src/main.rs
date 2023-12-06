
use nom::{
  IResult,
  bytes::complete::tag,
  character::complete::alpha1,
  multi::separated_list1,
  sequence::tuple,
};

#[derive(Debug, PartialEq)]
struct CubeReveal {
  red: i32,
  green: i32,
  blue: i32,
}

#[derive(Debug, PartialEq)]
struct CubeGame {
  id: i32,
  rev: Vec<CubeReveal>,
}

fn nom_i32(input: &str) -> IResult<&str, i32> {
  nom::character::complete::i32(input)
}

fn nom_color_reveal(input: &str) -> IResult<&str, (i32, &str)> {
  let (input, (num, _, col)) = tuple((nom_i32, tag(" "), alpha1))(input)?;

  Ok((input, (num, col)))
}

fn nom_cube_reveal(input: &str) -> IResult<&str, CubeReveal> {
  let (input, v) = separated_list1(tag(", "), nom_color_reveal)(input)?;

  let mut red: i32 = 0;
  let mut green: i32 = 0;
  let mut blue: i32 = 0;

  for x in &v {
    if x.1 == "red" { red = x.0; }
    if x.1 == "green" { green = x.0; }
    if x.1 == "blue" { blue = x.0; }
  }

  Ok((input, CubeReveal { red, green, blue }))
}

fn nom_cube_reveals(input: &str) -> IResult<&str, Vec<CubeReveal>> {
  let (input, v) = separated_list1(tag("; "), nom_cube_reveal)(input)?;

  Ok((input, v))
}

fn nom_game_id(input: &str) -> IResult<&str, i32> {
  let (input, _) = tag("Game ")(input)?;
  let (input, id) = nom_i32(input)?;
  let (input, _) = tag(": ")(input)?;

  Ok((input, id))
}

fn nom_game(input: &str) -> IResult<&str, CubeGame> {
  let (input, id) = nom_game_id(input)?;
  let (input, rev) = nom_cube_reveals(input)?;

  Ok((input, CubeGame { id, rev }))
}

fn parse_game(input: &str) -> CubeGame {
  match nom_game(input).ok() {
    Some((_, x)) => x,
    _ => panic!("Game could not be parsed")
  }
}

pub fn part1() -> String {
  let input = include_str!("../input1.txt");
  let lines = shared::input_as_lines(input);
  
  let games: Vec<CubeGame> = 
    lines.iter().map(|x| parse_game(x)).collect();
  
  games.iter()
    .filter(|g| g.rev.iter().all(|a| a.red <= 12 && a.green <= 13 && a.blue <= 14))
    .map(|g| g.id)
    .sum::<i32>()
    .to_string()
}

pub fn part2() -> String {
  let input = include_str!("../input1.txt");
  let lines = shared::input_as_lines(input);

  let games: Vec<CubeGame> = 
    lines.iter().map(|x| parse_game(x)).collect();
  
  games.iter()
    .map(|g| {
      let max_red = g.rev.iter().map(|a| a.red).max().unwrap();
      let max_green = g.rev.iter().map(|a| a.green).max().unwrap();
      let max_blue = g.rev.iter().map(|a| a.blue).max().unwrap();
      max_red * max_green * max_blue
    })
    .sum::<i32>()
    .to_string()
}

fn main() {
  println!("{}", part1());
  println!("{}", part2());
}

#[cfg(test)]
mod tests {
  #[test]
  fn solve_part1() {
    assert_eq!(super::part1(), "2377");
  }

  #[test]
  fn solve_part2() {
    assert_eq!(super::part2(), "71220");
  }
}
