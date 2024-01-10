use std::collections::HashSet;

fn calc_winnings01(input: &str) -> i32 {
    let mut tot: i32 = 0;
    for line in input.lines() {
        let Some((_, nums)) = line.split_once(":") else {
            continue;
        };
        let Some((winnings, mine)) = nums.split_once("|") else {
            continue;
        };

        let winnings: HashSet<&str> = winnings.split(" ").collect();
        let mine: HashSet<&str> = mine.split(" ").collect();

        let count: u32 = mine.into_iter().filter(|f| winnings.contains(f)).count() as u32;
        if count >= 2 {
            let score = 2_i32.pow(count - 2);
            tot += score;
        }
    }
    tot
}

fn calc_winnings02(input: &str) -> i32 {
    let len = input.lines().count();
    let mut vec = vec![1; len];
    let mut tot = 0;
    for (i, line) in input.lines().enumerate() {
        let Some((_, vals)) = line.split_once(":") else {
            tot += vec[i];
            continue;
        };
        let Some((left, right)) = vals.split_once("|") else {
            tot += vec[i];
            continue;
        };

        let winnings: HashSet<&str> = left.split(" ").collect();
        let mine: HashSet<&str> = right.split(" ").collect();

        let count = mine.into_iter().filter(|f| winnings.contains(f)).count();

        for j in i..(i + count - 1) {
            vec[j + 1] += vec[i];
        }
        tot += vec[i];
    }
    println!("{:?}", vec);
    tot
}

fn main() {
    let input = include_str!("test-01.txt");
    let ex = include_str!("example-01.txt");
    let tot1 = calc_winnings01(input);
    let tot2 = calc_winnings02(input);
    println!("Part 01 answ: {}", tot1);
    println!("Part 02 answ: {}", tot2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01() {
        let input = include_str!("example-01.txt");
        assert_eq!(13, calc_winnings01(input));
    }

    #[test]
    fn part02() {
        let input = include_str!("example-01.txt");
        assert_eq!(30, calc_winnings02(input));
    }
}
