use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let filename: &str = "input/input_1.txt";
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let l: Vec<&str> = line.split_whitespace().collect();
        let word1: i32 = l[0].parse::<i32>().unwrap();
        let word2: i32 = l[1].parse::<i32>().unwrap();
        column1.push(word1);
        column2.push(word2);
    }
    column1.sort();
    column2.sort();

    // Part 1
    let mut sol1: i32 = 0;
    for (i, j) in column1.clone().into_iter().zip(column2.clone().into_iter()) {
        sol1 += (i - j).abs();
    }

    // Part 2
    let mut sol2: i32 = 0;
    for i in &column1 {
        for j in &column2 {
            if j == i {
                sol2 += i
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
