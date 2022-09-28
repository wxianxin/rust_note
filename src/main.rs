#![allow(unused)] // suppress warnings for unused variables

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("Hello, world!"); // println is macro (with "!")
    println!("What's your name?");
    let mut name = String::new(); // in rust by default the variables defined are immutable
    let greeting = "Nice to meet you.";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input.");
    println!("Hello {}! {}", name.trim_end(), greeting);

    //----------------------------------------------------------------------------------

    const ONE_BIL: u32 = 1_000_000_000;
    const PI: f32 = 3.1415926;
    let age = "30";
    // shadowing: define variables with the same name but different types
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_BIL);
    // shadowing: define variables with the same name but different types

    // datatype
    /*  integer
        Length	Signed	Unsigned
        8-bit	i8	u8
        16-bit	i16	u16
        32-bit	i32	u32
        64-bit	i64	u64
        128-bit	i128	u128
        arch	isize	usize       // using this will assign the length of computer architecture
    */

    println!("Max u32: {}", u32::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);

    let is_true: bool = true;
    let my_grade: char = 'A';

    //----------------------------------------------------------------------------------
    // math
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);

    let random_num = rand::thread_rng().gen_range(1..101); // [1, 101) -> [1, 100]
    println!("random integer from 1 - 100: {}", random_num);

    //----------------------------------------------------------------------------------
    // condition
    let age: i32 = 30;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday")
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday")
    } else {
        println!("Not an Important Birthday")
    }

    // ternary operator
    let mut my_age = 18;
    let can_vote = if my_age >= 18 { true } else { false }; // no ':' after 'true' because this is
                                                            // an assignment instead of statement
    println!("Can Vote: {}", can_vote);

    // match
    let age2 = 9;
    match age2 {
        1..=18 => println!("Important Birthday"), // 1..18: [1, 18)  1..=18: [1, 18]
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

    let my_age = 19;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote."),
    };

    //----------------------------------------------------------------------------------
    // array, elements in array must be of the same datatype, array has fixed size
    let arr_1: [i32; 4] = [1, 2, 3, 4];
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st: {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    let mut loop_idx = 0;
    // loop
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 7 {
            break;
        }
        println!("Val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    let mut loop_idx = 0;
    // while
    while loop_idx < arr_1.len() {
        println!("arr_1[{}]: {}", loop_idx, arr_1[loop_idx]);
        loop_idx += 1;
    }

    // for
    for x in arr_1.iter() {
        println!("Val: {}", x)
    }

    // tuple: contain multiple datatypes
    
}
