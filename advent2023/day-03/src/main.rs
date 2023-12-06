
#[derive(Debug)]
struct EngineComponent {
  code: char,
  is_symbol: bool,
  is_digit: bool,
  part: u32,
  ratio: u32,
}

#[derive(Debug)]
struct Engine {
  schematic: Vec<Vec<EngineComponent>>
}

impl Engine {
  fn len_x(&self) -> usize {
    self.schematic.iter().map(|x| x.len()).max().unwrap_or(0)
  }
  fn len_y(&self) -> usize {
    self.schematic.len()
  }

  fn is_symbol(&self, x: i32, y: i32) -> bool {
    self.get(x, y).is_some_and(|x| x.is_symbol)
  }

  fn is_digit(&self, x: i32, y: i32) -> bool {
    self.get(x, y).is_some_and(|x| x.is_digit)
  }

  fn near_symbol(&self, x: i32, y: i32) -> bool {
    self.is_symbol(x - 1, y - 1) ||
    self.is_symbol(x - 1, y) ||
    self.is_symbol(x - 1, y + 1) ||
    self.is_symbol(x, y - 1) ||
    self.is_symbol(x, y + 1) ||
    self.is_symbol(x + 1, y - 1) ||
    self.is_symbol(x + 1, y) ||
    self.is_symbol(x + 1, y + 1)
  }

  fn find_part_from(&self, x: i32, y: i32) -> Option<(i32, i32)> {
    if let Some(c) = self.get(x, y) {
      if c.part > 0 {
        return Some((x, y));
      }
      if c.is_digit {
        return self.find_part_from(x - 1, y); 
      }
    }
    None
  }

  fn test_part(&self, x: i32, y: i32) -> Option<(String, bool)> {
    if let Some(c) = self.get(x, y) {
      if c.is_digit {
        let id = c.code.to_digit(10).unwrap();
        let sym = self.near_symbol(x, y);

        if let Some(c1) = self.test_part(x + 1, y) {
          let id2: String = format!("{}{}", id, c1.0);
          let sym2: bool = sym || c1.1;
          return Some((id2, sym2));
        }
        return Some((id.to_string(), sym));
      }
    }
    None
  }

  fn test_ratio(&self, x: i32, y: i32) -> Option<u32> {
    let mut parts: Vec<(i32,i32)> = vec![];

    if let Some(c) = self.get(x, y) {
      if c.is_symbol && c.code == '*' {
        let n = self.get(x, y - 1);
        let s = self.get(x, y + 1);

        if n.is_some_and(|x| x.is_digit) {
          if let Some(cn2) = self.find_part_from(x, y - 1) { parts.push(cn2); }
        } else {
          if let Some(cn1) = self.find_part_from(x - 1, y - 1) { parts.push(cn1); }
          if let Some(cn3) = self.find_part_from(x + 1, y - 1) { parts.push(cn3); }
        }

        if s.is_some_and(|x| x.is_digit) {
          if let Some(cs2) = self.find_part_from(x, y + 1) { parts.push(cs2); }
        } else {
          if let Some(cs1) = self.find_part_from(x - 1, y + 1) { parts.push(cs1); }
          if let Some(cs3) = self.find_part_from(x + 1, y + 1) { parts.push(cs3); }
        }

        if let Some(ce) = self.find_part_from(x + 1, y) { parts.push(ce); }
        if let Some(cw) = self.find_part_from(x - 1, y) { parts.push(cw); }

        if parts.len() == 2 {
          let p1 = self.get(parts[0].0, parts[0].1).unwrap();
          let p2 = self.get(parts[1].0, parts[1].1).unwrap();
          return Some(p1.part * p2.part);
        }
      }
    }
    None
  }

  fn get(&self, x: i32, y: i32) -> Option<&EngineComponent> {
    if x < 0 || y < 0 { return None; }
    if let Some(cy) = self.schematic.get(y as usize) {
      return cy.get(x as usize);
    }
    None
  }

  fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut EngineComponent> {
    if x < 0 || y < 0 { return None; }
    if let Some(cy) = self.schematic.get_mut(y as usize) {
      return cy.get_mut(x as usize);
    }
    None
  }
}

impl From<Vec<String>> for Engine {
    fn from(item: Vec<String>) -> Self {
      fn is_digit(ch: char) -> bool { "0123456789".contains(ch) }
      fn is_symbol(ch: char) -> bool { "@#$%&*-+=/".contains(ch) }

      fn calc_schematic(eng: &mut Engine, items: &[String]) {
        eng.schematic = 
          items.iter()
          .map(|x| {
            x.chars()
            .map(|ch| EngineComponent {
              code: ch,
              is_symbol: is_symbol(ch),
              is_digit: is_digit(ch),
              part: 0,
              ratio: 0,
            }).collect()
          }).collect()        
      }

      fn calc_parts(eng: &mut Engine) {
        for j in 0..eng.len_y() {
          let jj = j as i32;
          for i in 0..eng.len_x() {
            let ii = i as i32;
            if let Some(t0) = eng.test_part(ii, jj) {
              if t0.1 && !eng.is_digit(ii - 1, jj) {
                //println!("test_part({ii},{jj}) => {t0:?}");
                let c0 = eng.get_mut(ii, jj).unwrap();
                c0.part = (t0.0).parse::<u32>().unwrap();
              }
            }
          }
        }
      }

      fn calc_gears(eng: &mut Engine) {
        for j in 0..eng.len_y() {
          let jj = j as i32;
          for i in 0..eng.len_x() {
            let ii = i as i32;
            if let Some(t0) = eng.test_ratio(ii, jj) {
              //println!("test_ratio({ii},{jj}) => {t0:?}");
              let c0 = eng.get_mut(ii, jj).unwrap();
              c0.ratio = t0;
            }
          }
        }
      }

      let mut eng = Engine {
        schematic: vec![],
      };
      calc_schematic(&mut eng, &item);
      calc_parts(&mut eng);
      calc_gears(&mut eng);

      eng
    }
}

pub fn part1() -> String {
  let input = include_str!("../input1.txt");
  let lines = shared::input_as_lines(input);
  let eng: Engine = lines.into();
  
  eng.schematic.iter()
    .flatten()
    .map(|x| x.part)
    .sum::<u32>()
    .to_string()
}

pub fn part2() -> String {
  let input = include_str!("../input1.txt");
  let lines = shared::input_as_lines(input);
  let eng: Engine = lines.into();

  eng.schematic.iter()
    .flatten()
    .map(|x| x.ratio)
    .sum::<u32>()
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
    assert_eq!(super::part1(), "550064");
  }

  #[test]
  fn solve_part2() {
    assert_eq!(super::part2(), "85010461");
  }
}
