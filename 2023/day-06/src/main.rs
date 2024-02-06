use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
}

impl Race {
    fn num_wins(self: &Self) -> i32 {
        (0..=self.time)
            .filter(|t| t * (self.time - t) > self.distance)
            .count() as i32
    }
}

impl Races {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let Some(mut time) = lines.next() else {
            panic!("Could not get the first line");
        };
        let Some(mut distance) = lines.next() else {
            panic!("Could not get the second line");
        };
        (_, time) = time.split_once(":").unwrap();
        (_, distance) = distance.split_once(":").unwrap();

        let times: Vec<i32> = time
            .trim()
            .split(" ")
            .filter(|f| !f.is_empty())
            .map(|f| f.parse().unwrap())
            .collect();

        let distances: Vec<i32> = distance
            .trim()
            .split(" ")
            .filter(|f| !f.is_empty())
            .map(|f| f.parse().unwrap())
            .collect();

        Races {
            races: zip(times, distances)
                .map(|(t, d)| Race {
                    time: t,
                    distance: d,
                })
                .collect(),
        }
    }
}

fn calc_combinations(input: &str) -> i32 {
    let races = Races::from(input);
    races.races.iter().map(|r| r.num_wins()).product()
}

fn main() {
    let ex = include_str!("test1.txt");
    let answ1 = calc_combinations(ex);
    println!("{}", answ1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01() {
        let ex = include_str!("example1.txt");
        let answ1 = calc_combinations(ex);
        assert_eq!(answ1, 288);
    }
}
