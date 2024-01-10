fn main() {
    let txt = include_str!("./01.txt");
    let mut sum = 0;
    for line in txt.lines() {
        let digits: Vec<char> = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<char>>();
        if digits.len() < 1 {
            continue;
        }
        let val = format!("{}{}", digits[0], digits.last().unwrap_or(&'0'));
        sum += val.parse().unwrap_or(0);
    }
    println!("{sum}");
}
