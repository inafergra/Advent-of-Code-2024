use crate::{Solution, SolutionPair};
use std::fs::{read_to_string, File};
use itertools::Itertools;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    fn is_safe(report:&Vec<i16>)->bool {
        let mut i0: &i16 = &report[0];
        let direction = &report[1] - i0;
        let mut safe: bool = true;        
        for i in &report[1..] {
            let diff = i-i0;
            if diff.abs() <1 {safe = false}
            else if diff.abs()>3 {safe=false}
            else if direction*diff <0 {safe = false}
            
            if safe==false {break}
            i0 = i;
        }
        safe
    }


    let filename: &str= "input/input_2.txt";
    let mut sol1: i16 = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let report :Vec<i16>= line.split_whitespace().map(|s| s.parse::<i16>().unwrap()).collect();

        if is_safe(&report) {sol1 +=1; continue }
        println!("{}", "Report:");
        println!("{:?}", report);

        let combs = report.iter().combinations(report.len()-1);
        for comb in combs {
            let new_report: Vec<i16> = comb.iter().map(|&&x| x).collect();
            println!("{:?}", new_report);
            if is_safe(&new_report) {sol1 +=1; break}
        }
    }

    // Your solution here...
    // let sol1: u64 = n_safe_reports;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
