use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
pub fn solve() -> SolutionPair {
    let filename: &str = "input/input_3.txt";
    let memory: String = read_to_string(filename).unwrap();

    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sol1: i32 = 0;
    for captures in re.captures_iter(&memory.as_str()) {
        let num1: i32 = captures
            .get(1)
            .map_or("0", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let num2: i32 = captures
            .get(2)
            .map_or("0", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        sol1 += num1 * num2;
    }

    let re_2: Regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut sol2: i32 = 0;
    let mut enable: bool = true;
    for captures in re_2.captures_iter(&memory.as_str()) {
        let raw_match: &str = captures.get(0).map_or("0", |m| m.as_str());
        if raw_match == "do()" {
            enable = true
        } else if raw_match == "don't()" {
            enable = false
        }

        if enable {
            let num1: i32 = captures
                .get(1)
                .map_or("0", |m| m.as_str())
                .parse::<i32>()
                .unwrap();
            let num2: i32 = captures
                .get(2)
                .map_or("0", |m| m.as_str())
                .parse::<i32>()
                .unwrap();
            sol2 += num1 * num2;
        }
    }
    // for captures in re.captures_iter(memory.as_str()) {
    //     // Destructure the capture groups directly
    //     let num1: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
    //     let num2: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

    //     // Calculate the product
    //     let product = num1 * num2;

    //     println!("First number: {}, Second number: {}, Product: {}", num1, num2, product);
    // }

    // Your solution here...
    // let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
