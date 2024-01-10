use core::fmt;
use regex::{Error, Regex};
use std::vec;

struct CubeGame {
    r: Vec<i32>,
    b: Vec<i32>,
    g: Vec<i32>,
    game_id: i32,
}

impl fmt::Display for CubeGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Red: {:?}, Green: {:?}, Blue: {:?}, Game: {}",
            self.r, self.g, self.b, self.game_id
        )
    }
}

impl CubeGame {
    fn from_line(line: &str) -> Result<CubeGame, Error> {
        let re = Regex::new(r"((\d+) (blue|red|green))")?;
        let mut r: Vec<i32> = vec![];
        let mut b: Vec<i32> = vec![];
        let mut g: Vec<i32> = vec![];

        for m in re.captures_iter(line) {
            let val: i32 = m.get(2).unwrap().as_str().parse().unwrap();
            match m.get(3).unwrap().as_str() {
                "red" => r.append(&mut vec![val]),
                "blue" => b.append(&mut vec![val]),
                "green" => g.append(&mut vec![val]),
                _ => println!("Color did not match {}", m.get(3).unwrap().as_str()),
            }
        }

        let gre = Regex::new(r"Game (\d+)")?;
        let game_id: i32 = gre
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        Ok(CubeGame { r, g, b, game_id })
    }

    fn fits_rules(&self, mr: &i32, mb: &i32, mg: &i32) -> bool {
        self.r.iter().max().unwrap() <= mr
            && self.b.iter().max().unwrap() <= mb
            && self.g.iter().max().unwrap() <= mg
    }
    fn min_viable_amount(&self) -> i32 {
        let min_r = self.r.iter().max().unwrap();
        let min_b = self.b.iter().max().unwrap();
        let min_g = self.g.iter().max().unwrap();
        min_r * min_b * min_g
    }
}

fn main() {
    let input = include_str!("./01.txt");
    let mut total = 0;
    for line in input.lines() {
        let cube = CubeGame::from_line(line).unwrap();
        // if cube.fits_rules(&12, &14, &13) {
        //     total += cube.game_id;
        // }
        total += cube.min_viable_amount();
    }
    println!("{total}")
}
