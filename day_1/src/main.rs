use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
fn main() {
    let output = read_lines("input.txt");
    let mut sum = 0;
    for line in output {
        let last = line
            .chars()
            .filter(|c| c.is_numeric())
            .last()
            .to_owned()
            .unwrap()
            .to_string();
        let first = line
            .chars()
            .filter(|c| c.is_numeric())
            .nth(0)
            .to_owned()
            .unwrap()
            .to_string();
        let digis = format!("{}{}", first, last).parse().unwrap_or(0);
        sum += digis;
        // println!("Running total: {}", sum);
    }
    println!("Total: {}", sum)
}
