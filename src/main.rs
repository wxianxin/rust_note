#![allow(unused)]   // suppress warnings for unused variables

use std::io;
// use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");  // println is macro (with "!")
    println!("What's your name?");
    let mut name = String::new();   // in rust by default the variables defined are immutable
    let greeting = "Nice to meet you.";
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input.");
    println!("Hello {}! {}", name.trim_end(), greeting);

}
