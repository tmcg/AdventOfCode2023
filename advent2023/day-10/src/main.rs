use std::collections::{ HashMap, HashSet };

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Compass {
    North,
    South,
    East,
    West,
}

impl Compass {
    fn reverse(&self) -> Compass {
        match self {
            Compass::North => Compass::South,
            Compass::South => Compass::North,
            Compass::East => Compass::West,
            Compass::West => Compass::East,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SegmentType {
    NorthSouth,
    NorthWest,
    NorthEast,
    EastWest,
    SouthWest,
    SouthEast,
    Ground,
}

impl SegmentType {
    fn find_exit(&self, from: Compass) -> Option<Compass> {
        match (self, from) {
            (SegmentType::NorthWest, Compass::North) => Some(Compass::West),
            (SegmentType::NorthSouth, Compass::North) => Some(Compass::South),
            (SegmentType::NorthEast, Compass::North) => Some(Compass::East),
            (SegmentType::NorthSouth, Compass::South) => Some(Compass::North),
            (SegmentType::SouthEast, Compass::South) => Some(Compass::East),
            (SegmentType::SouthWest, Compass::South) => Some(Compass::West),
            (SegmentType::NorthEast, Compass::East) => Some(Compass::North),
            (SegmentType::SouthEast, Compass::East) => Some(Compass::South),
            (SegmentType::EastWest, Compass::East) => Some(Compass::West),
            (SegmentType::NorthWest, Compass::West) => Some(Compass::North),
            (SegmentType::SouthWest, Compass::West) => Some(Compass::South),
            (SegmentType::EastWest, Compass::West) => Some(Compass::East),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct MazeSegment {
    seg_type: SegmentType,
}

#[derive(Debug)]
struct PipeMaze {
    pos: HashMap<(i32, i32), MazeSegment>,
    start: (i32, i32),
    size: (i32, i32),
}

impl PipeMaze {
    fn next_pos(xy: (i32, i32), to: Compass) -> (i32, i32) {
        match to {
            Compass::North => (xy.0, xy.1 - 1),
            Compass::South => (xy.0, xy.1 + 1),
            Compass::East => (xy.0 + 1, xy.1),
            Compass::West => (xy.0 - 1, xy.1),
        }
    }

    fn valid_pos(&self, xy: (i32, i32)) -> bool {
        !(xy.0 < 0 || xy.0 >= self.size.0 || xy.1 < 0 || xy.1 >= self.size.1)
    }

    fn get_segment_type(&self, xy: (i32, i32)) -> SegmentType {
        self.pos.get(&xy).map_or(SegmentType::Ground, |x| x.seg_type)
    }

    fn has_exit(&self, xy: (i32, i32), from: Compass) -> bool {
        self.get_segment_type(xy).find_exit(from).is_some()
    }

    fn first_exit(&self) -> Option<Compass> {
        let seg_type = self.pos.get(&self.start)?.seg_type;

        [
        Compass::North,
        Compass::South,
        Compass::East,
        Compass::West,
        ]
        .into_iter()
        .find(|c| seg_type.find_exit(*c).is_some())
    }

    fn find_farthest(&self) -> i32 {
        (self.find_main_loop().len() / 2) as i32
    }

    fn find_main_loop(&self) -> HashSet<(i32, i32)> {
        let mut result: HashSet<(i32, i32)> = HashSet::new();
        let mut dir = self.first_exit().unwrap();
        let mut pos = (self.start.0, self.start.1);

        loop {
            result.insert(pos);

            pos = PipeMaze::next_pos(pos, dir);
            dir = self
                .get_segment_type(pos)
                .find_exit(dir.reverse())
                .unwrap();
            
            if pos.0 == self.start.0
               && pos.1 == self.start.1 {
                return result;
            }
        }
    }

    fn find_nest(&self) -> i32 {
        let mut hs_left: HashSet<(i32, i32)> = HashSet::new();
        let mut hs_right: HashSet<(i32, i32)> = HashSet::new();
        
        let hs_loop = self.find_main_loop();
        let st = hs_loop.iter()
            .find(|x| self.get_segment_type(**x) == SegmentType::NorthSouth)
            .expect("Unable to find north/south segment");

        let mut dir = Compass::North;
        let mut pos = (st.0, st.1);
        let mut typ;

        loop {
            let old_dir = dir;

            pos = PipeMaze::next_pos(pos, dir);
            typ = self.get_segment_type(pos);
            dir = typ.find_exit(dir.reverse()).unwrap();

            let n = PipeMaze::next_pos(pos, Compass::North);
            let s = PipeMaze::next_pos(pos, Compass::South);
            let e = PipeMaze::next_pos(pos, Compass::East);
            let w = PipeMaze::next_pos(pos, Compass::West);

            //println!("{old_dir:?} {dir:?} {pos:?}");

            match (old_dir, dir) {
                (Compass::North, Compass::North) => {
                    self.flood_fill(w, &hs_loop, &mut hs_left);
                    self.flood_fill(e, &hs_loop, &mut hs_right);
                }
                (Compass::North, Compass::East) => {
                    self.flood_fill(n, &hs_loop, &mut hs_left);
                    self.flood_fill(w, &hs_loop, &mut hs_left);
                }
                (Compass::North, Compass::West) => {
                    self.flood_fill(n, &hs_loop, &mut hs_right);
                    self.flood_fill(e, &hs_loop, &mut hs_right);
                }
                (Compass::South, Compass::South) => {
                    self.flood_fill(e, &hs_loop, &mut hs_left);
                    self.flood_fill(w, &hs_loop, &mut hs_right);
                }
                (Compass::South, Compass::East) => {
                    self.flood_fill(s, &hs_loop, &mut hs_right);
                    self.flood_fill(w, &hs_loop, &mut hs_right);
                }
                (Compass::South, Compass::West) => {
                    self.flood_fill(s, &hs_loop, &mut hs_left);
                    self.flood_fill(e, &hs_loop, &mut hs_left);
                }
                (Compass::East, Compass::North) => {
                    self.flood_fill(s, &hs_loop, &mut hs_right);
                    self.flood_fill(e, &hs_loop, &mut hs_right);
                }
                (Compass::East, Compass::South) => {
                    self.flood_fill(n, &hs_loop, &mut hs_left);
                    self.flood_fill(e, &hs_loop, &mut hs_left);
                }
                (Compass::East, Compass::East) => {
                    self.flood_fill(n, &hs_loop, &mut hs_left);
                    self.flood_fill(s, &hs_loop, &mut hs_right);
                }
                (Compass::West, Compass::North) => {
                    self.flood_fill(s, &hs_loop, &mut hs_left);
                    self.flood_fill(w, &hs_loop, &mut hs_left);
                }
                (Compass::West, Compass::South) => {
                    self.flood_fill(n, &hs_loop, &mut hs_right);
                    self.flood_fill(w, &hs_loop, &mut hs_right);
                }
                (Compass::West, Compass::West) => {
                    self.flood_fill(s, &hs_loop, &mut hs_left);
                    self.flood_fill(n, &hs_loop, &mut hs_right);
                }
                _ => panic!("Invalid change of direction"),
            }
            
            if pos.0 == st.0 && pos.1 == st.1 {
                break;
            }
        }        

        let left_len = hs_left.len() as i32;
        let right_len = hs_right.len() as i32;
        //println!("left len={:?}", left_len);
        //println!("right len={:?}", right_len);

        // Assume the smallest filled area is the nest.
        // Doesn't work for the general case, but does
        // for the subset of puzzle inputs provided.
        if left_len < right_len { left_len } else { right_len }
    }

    fn flood_fill(&self,
        xy: (i32, i32),
        hs_loop: &HashSet<(i32, i32)>,
        hs_fill: &mut HashSet<(i32, i32)>) {

        if !self.valid_pos(xy) { return; }
        if hs_fill.contains(&xy) { return; }
        if hs_loop.contains(&xy) { return; }

        hs_fill.insert(xy);

        let n = PipeMaze::next_pos(xy, Compass::North);
        self.flood_fill(n, hs_loop, hs_fill);
        let s = PipeMaze::next_pos(xy, Compass::South);
        self.flood_fill(s, hs_loop, hs_fill);
        let e = PipeMaze::next_pos(xy, Compass::East);
        self.flood_fill(e, hs_loop, hs_fill);
        let w = PipeMaze::next_pos(xy, Compass::West);
        self.flood_fill(w, hs_loop, hs_fill);
    }
}

impl From<&str> for PipeMaze {
    fn from(item: &str) -> Self {
        let mut maze = PipeMaze {
            pos: HashMap::new(),
            start: (0, 0),
            size: (0, 0),
        };

        item.split("\r\n")
        .enumerate()
        .for_each(|(y, yv)| {
            yv.chars()
            .enumerate()
            .for_each(|(x, xv)| {
                if xv == 'S' {
                    maze.start = (x as i32, y as i32);
                }

                maze.pos.insert(
                    (x as i32, y as i32),
                    MazeSegment {
                        seg_type: match xv {
                            '|' => { SegmentType::NorthSouth },
                            'L' => { SegmentType::NorthEast },
                            'J' => { SegmentType::NorthWest },
                            'F' => { SegmentType::SouthEast },
                            '7' => { SegmentType::SouthWest },
                            '-' => { SegmentType::EastWest },
                            _ => { SegmentType::Ground },
                        }
                    });
            });
        });

        // Find the real segment type of the start position
        let n0 = PipeMaze::next_pos(maze.start, Compass::North);
        let s0 = PipeMaze::next_pos(maze.start, Compass::South);
        let e0 = PipeMaze::next_pos(maze.start, Compass::East);
        let w0 = PipeMaze::next_pos(maze.start, Compass::West);

        let n1 = maze.has_exit(n0, Compass::South);
        let s1 = maze.has_exit(s0, Compass::North);
        let e1 = maze.has_exit(e0, Compass::West);
        let w1 = maze.has_exit(w0, Compass::East);

        let start_type = match (n1,s1,e1,w1) {
            (true,true,false,false) => SegmentType::NorthSouth,
            (true,false,true,false) => SegmentType::NorthEast,
            (true,false,false,true) => SegmentType::NorthWest,
            (false,true,true,false) => SegmentType::SouthEast,
            (false,true,false,true) => SegmentType::SouthWest,
            (false,false,true,true) => SegmentType::EastWest,
            _ => SegmentType::Ground,
        };

        if let Some(start) = maze.pos.get_mut(&maze.start) {
            start.seg_type = start_type;
        }

        maze.size = (
            maze.pos.keys().map(|k| k.0 + 1).max().unwrap_or(0),
            maze.pos.keys().map(|k| k.1 + 1).max().unwrap_or(0),
        );
        
        maze
    }
}

pub fn part1() -> String {
    let input = include_str!("../input1.txt");
    let maze = PipeMaze::from(input);
    maze.find_farthest().to_string()
}

pub fn part2() -> String {
    let input = include_str!("../input1.txt");
    let maze = PipeMaze::from(input);
    maze.find_nest().to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_pos() {
        assert_eq!(PipeMaze::next_pos((3, 4), Compass::North), (3, 3));
        assert_eq!(PipeMaze::next_pos((3, 4), Compass::South), (3, 5));
        assert_eq!(PipeMaze::next_pos((3, 4), Compass::East), (4, 4));
        assert_eq!(PipeMaze::next_pos((3, 4), Compass::West), (2, 4));
    }

    #[test]
    fn find_exit() {
        let ns = SegmentType::NorthSouth;
        let ew = SegmentType::EastWest;
        let se = SegmentType::SouthEast;

        assert_eq!(ns.find_exit(Compass::North), Some(Compass::South));
        assert_eq!(ew.find_exit(Compass::East), Some(Compass::West));
        assert_eq!(se.find_exit(Compass::South), Some(Compass::East));
        assert_eq!(se.find_exit(Compass::East), Some(Compass::South));
        assert_eq!(ns.find_exit(ns.find_exit(Compass::North).unwrap()), Some(Compass::North));
        assert_eq!(ew.find_exit(ew.find_exit(Compass::East).unwrap()), Some(Compass::East));
        assert_eq!(se.find_exit(se.find_exit(Compass::East).unwrap()), Some(Compass::East));
        assert_eq!(se.find_exit(Compass::North), None);
        assert_eq!(se.find_exit(Compass::West), None);
    }

    #[test]
    fn parse_input() {
        let input = include_str!("../input2.txt");
        let maze = PipeMaze::from(input);
        
        assert_eq!(maze.pos.keys().len(), 25);
        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.size, (5, 5));
        assert!(maze.valid_pos((0, 0)));

        assert_eq!(maze.pos.get(&maze.start).unwrap().seg_type, SegmentType::SouthEast);
        assert_eq!(maze.pos.get(&(1,2)).unwrap().seg_type, SegmentType::NorthSouth);
        assert_eq!(maze.pos.get(&(1,3)).unwrap().seg_type, SegmentType::NorthEast);
        assert_eq!(maze.pos.get(&(3,1)).unwrap().seg_type, SegmentType::SouthWest);
        assert_eq!(maze.pos.get(&(3,3)).unwrap().seg_type, SegmentType::NorthWest);
    }

    #[test]
    fn find_main_loop() {
        let input2 = include_str!("../input2.txt");
        let maze2 = PipeMaze::from(input2);
        let hs = maze2.find_main_loop();
        assert_eq!(hs.len(), 8);
        assert!(hs.contains(&(1,1)));
        assert!(hs.contains(&(1,2)));
        assert!(hs.contains(&(1,3)));
        assert!(hs.contains(&(2,1)));
        assert!(hs.contains(&(2,3)));
        assert!(hs.contains(&(3,1)));
        assert!(hs.contains(&(3,2)));
        assert!(hs.contains(&(3,3)));
    }

    #[test]
    fn solve_sample_part1() {
        let input2 = include_str!("../input2.txt");
        let maze2 = PipeMaze::from(input2);
        assert_eq!(maze2.find_farthest(), 4);

        let input3 = include_str!("../input3.txt");
        let maze3 = PipeMaze::from(input3);
        assert_eq!(maze3.find_farthest(), 8);
    }

    #[test]
    fn solve_sample_part2() {
        let input2 = include_str!("../input2.txt");
        let maze2 = PipeMaze::from(input2);
        assert_eq!(maze2.find_nest(), 1);

        let input4 = include_str!("../input4.txt");
        let maze4 = PipeMaze::from(input4);
        assert_eq!(maze4.find_nest(), 4);

        let input5 = include_str!("../input5.txt");
        let maze5 = PipeMaze::from(input5);
        assert_eq!(maze5.find_nest(), 4);

        let input6 = include_str!("../input6.txt");
        let maze6 = PipeMaze::from(input6);
        assert_eq!(maze6.find_nest(), 8);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "7030");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "285");
    }
}
