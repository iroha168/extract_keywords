use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::fs;
use std::io::{self, BufWriter, Write, BufReader, BufRead};

fn main(){

    let args: Vec<String> = env::args().collect();
    let paths = fs::read_dir("./result").unwrap();
    let args: Vec<String> = env::args().collect();
    let paths = fs::read_dir("./result").unwrap();
    for path in paths {
        let p =  &path.unwrap().path();
        let f =File::open(p).unwrap();
        let f = BufReader::new(f);
        //let f = File::open("1234565_code.txt").unwrap();
        for line in f.lines() {
            println!("{}", line.unwrap());
        }
    }
}
