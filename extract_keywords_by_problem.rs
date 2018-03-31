use std::prelude::*;
use std::io::{BufWriter, Write, BufReader, BufRead};
use std::fs::File;
use std::fs;
use std::collections::HashSet;

fn main(){
    let paths = fs::read_dir("./result/").unwrap();
    let mut problems = HashSet::new();
    for path in paths {
        let p = path.unwrap().path();
        let f = File::open(p).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines(){
            let ln = line.unwrap();
            problems.insert(ln);
        }
    }
    let mut f = BufWriter::new(fs::File::create("result.txt").unwrap());
    for problem in problems{
        f.write_all(problem.as_bytes()).unwrap();
    }
}


