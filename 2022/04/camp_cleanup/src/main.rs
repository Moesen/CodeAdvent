use std::fs;

fn main() {
    let contents = fs::read_to_string("./04-example.txt").expect("Could not read the file");
    for line in contents.lines() {
        let mut s = line.split(",");
        let (ex, ey): (i32, i32) = match s.next().unwrap().split("-") {
        }
    }
}
