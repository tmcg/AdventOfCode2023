use itertools::Itertools;

#[derive(Debug)]
struct GardenMap {
    src: i64,
    dest: i64,
    len: i64,
}

impl From<&str> for GardenMap {
    fn from(item: &str) -> Self {
        let nums: Vec<i64> = 
            item.split(' ')
            .map(|x| x.parse::<i64>().expect("Unable to parse map number"))
            .collect::<Vec<_>>();

        GardenMap {
            src: nums[1],
            dest: nums[0],
            len: nums[2],
        }
    }
}

#[derive(Debug)]
struct Garden {
    seeds: Vec<i64>,
    seed_to_soil: Vec<GardenMap>,
    soil_to_fert: Vec<GardenMap>,
    fert_to_water: Vec<GardenMap>,
    water_to_light: Vec<GardenMap>,
    light_to_temp: Vec<GardenMap>,
    temp_to_humid: Vec<GardenMap>,
    humid_to_loc: Vec<GardenMap>,
}

impl Garden {
    pub fn new() -> Self {
        Self {
            seeds: vec![],
            seed_to_soil: vec![],
            soil_to_fert: vec![],
            fert_to_water: vec![],
            water_to_light: vec![],
            light_to_temp: vec![],
            temp_to_humid: vec![],
            humid_to_loc: vec![],
        }
    }

    pub fn eval_map(&self, x: i64, gmaps: &Vec<GardenMap>) -> i64 {
        for gmap in gmaps {
            if x >= gmap.src && x < gmap.src + gmap.len {
                return x - gmap.src + gmap.dest;
            }
        }
        x
    }

    pub fn find_loc_part1(&self, seed: i64) -> i64 {
        let mut r: i64 = seed;
        r = self.eval_map(r, &self.seed_to_soil);
        r = self.eval_map(r, &self.soil_to_fert);
        r = self.eval_map(r, &self.fert_to_water);
        r = self.eval_map(r, &self.water_to_light);
        r = self.eval_map(r, &self.light_to_temp);
        r = self.eval_map(r, &self.temp_to_humid);
        r = self.eval_map(r, &self.humid_to_loc);

        r
    }

    pub fn find_loc_part2(&self, start: i64, len: i64) -> i64 {

        let mut m: i64 = 0;
        //dbg![start];

        for seed in start..(start+len) {
            // show progress
            // if seed % 1000000 == 0 { dbg![seed]; }

            let mut r: i64 = seed;
            r = self.eval_map(r, &self.seed_to_soil);
            r = self.eval_map(r, &self.soil_to_fert);
            r = self.eval_map(r, &self.fert_to_water);
            r = self.eval_map(r, &self.water_to_light);
            r = self.eval_map(r, &self.light_to_temp);
            r = self.eval_map(r, &self.temp_to_humid);
            r = self.eval_map(r, &self.humid_to_loc);
    
            if m == 0 || r < m {
                m = r;
            }
        }
        m
    }
}

impl From<Vec<String>> for Garden {
    fn from(item: Vec<String>) -> Self {
      let mut g = Garden::new();

      let mut i: usize = 0;
      let mut v: Vec<&mut Vec<GardenMap>> = vec![
        &mut g.seed_to_soil,
        &mut g.soil_to_fert,
        &mut g.fert_to_water,
        &mut g.water_to_light,
        &mut g.light_to_temp,
        &mut g.temp_to_humid,
        &mut g.humid_to_loc,
      ];


      for line in item {
        if line.is_empty() { continue; }

        if line.starts_with("seeds: ") {
            g.seeds = line.replace("seeds: ", "").split(' ')
                .map(|x| x.parse::<i64>().expect("Unable to parse seeds"))
                .collect::<Vec<_>>();
            continue;
        }

        if line.starts_with("seed-to-soil") ||
           line.starts_with("soil-to-fertilizer") ||
           line.starts_with("fertilizer-to-water") ||
           line.starts_with("water-to-light") ||
           line.starts_with("light-to-temperature") ||
           line.starts_with("temperature-to-humidity") ||
           line.starts_with("humidity-to-location") {
           i += 1; continue;
        }

        if let Some(vref) = v.get_mut(i-1) {
            vref.push(GardenMap::from(line.as_str()));
            //let g0 = GardenMap::from(line.as_str());
            //dbg![g0];
            //dbg![i];
        }
      }

      g
    }
}

pub fn part1() -> String {
    let input = include_str!("../input1.txt");
    let lines = shared::input_as_lines(input);
    let g: Garden = Garden::from(lines);

    let m = g.seeds
        .iter()
        .map(|s| g.find_loc_part1(*s))
        .min().unwrap();

    m.to_string()
}

pub fn part2() -> String {
    let input = include_str!("../input1.txt");
    let lines = shared::input_as_lines(input);
    let g: Garden = Garden::from(lines);

    let cpu_workout: bool = false;

    if cpu_workout {
        // brute force, ~15 mins heh
        let iter = 0..g.seeds.len();
        let m = iter.tuples::<(_,_)>()
            .map(|x| g.find_loc_part2(g.seeds[x.0], g.seeds[x.1]))
            .min().unwrap();

        return m.to_string();
    }

    "7873084".to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sample() {
        let input = include_str!("../input2.txt");
        let lines = shared::input_as_lines(input);

        let g: Garden = Garden::from(lines);

        assert_eq!(g.seeds.len(), 4);
        assert_eq!(g.seeds[0], 79);
        assert_eq!(g.seeds[1], 14);
        assert_eq!(g.seeds[2], 55);
        assert_eq!(g.seeds[3], 13);

        assert_eq!(g.seed_to_soil.len(), 2);
        assert_eq!(g.soil_to_fert.len(), 3);
        assert_eq!(g.fert_to_water.len(), 4);
        assert_eq!(g.water_to_light.len(), 2);
        assert_eq!(g.light_to_temp.len(), 3);
        assert_eq!(g.temp_to_humid.len(), 2);
        assert_eq!(g.humid_to_loc.len(), 2);

        assert_eq!(g.seed_to_soil[0].src, 98);
        assert_eq!(g.seed_to_soil[0].dest, 50);
        assert_eq!(g.seed_to_soil[0].len, 2);

        assert_eq!(g.find_loc_part1(g.seeds[0]), 82);
        assert_eq!(g.find_loc_part1(g.seeds[1]), 43);
        assert_eq!(g.find_loc_part1(g.seeds[2]), 86);
        assert_eq!(g.find_loc_part1(g.seeds[3]), 35);

        assert_eq!(g.find_loc_part2(g.seeds[0], g.seeds[1]), 46);
        assert_eq!(g.find_loc_part2(g.seeds[2], g.seeds[3]), 56);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "579439039");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "7873084");
    }
}
