use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn start() -> Result<()> {
    let mut sums = sums()?;
    println!("part 1: {}", sums.iter().max().unwrap());
    sums.sort_by(|a, b| b.cmp(a));
    println!("part 2: {}", sums.iter().take(3).sum::<u32>());
    Ok(())
}

fn sums() -> Result<Vec<u32>> {
    let file = File::open("input/1.txt")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    Ok(lines.iter().fold(Vec::new(), sum))
}

fn sum(mut result: Vec<u32>, element: &String) -> Vec<u32> {
    if element.is_empty() {
        result.push(0);
    } else {
        let value = result.pop().unwrap_or(0) + element.parse::<u32>().unwrap();
        result.push(value);
    }
    result
}
