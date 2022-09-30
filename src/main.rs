#![allow(unused)] // suppress warnings for unused variables

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("Hello, world!"); // println is macro (with "!")
    // Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
    // Macros are executed at compile time. They generally expand into new pieces of code that the compiler will then need to further process.
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

    //----------------------------------------------------------------------------------
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

    //----------------------------------------------------------------------------------
    // tuple: contain multiple datatypes in a list of a fixed size
    let my_tuple: (u8, String, f64) = (32, "Steven".to_string(), 10_0000.00);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);

    //----------------------------------------------------------------------------------
    // string
    // 2 types of string:
    //      String -> vector of bytes that can be changed
    //      &str -> point to the string and allow for viewing
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for x in st1.split_whitespace() {
        println!("{}", x);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3: String = String::from(" i n x k g d d d c y");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println! {"{}", char};
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6]; // a slice of string
    println!("String length: {}", st6.len());
    st5.clear(); // delete value from string if mut

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7; // after this operation, st6 is no longer available
    for char in st8.bytes() {
        println!("{}", char); // print unicode
    }

    //----------------------------------------------------------------------------------
    // casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    //----------------------------------------------------------------------------------
    // enum
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Friday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend!!!"),
        Day::Sunday => println!("Weekend!!!"),
    }

    println!("Is today the weekend ? {}", today.is_weekend());

    //----------------------------------------------------------------------------------
    // vector: like arrays but with variable size if mut, only store same type of data
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    println!("{:?}", vec2);
    vec2.push(5);
    println!("1st: {}", vec2[0]);
    let second: &i32 = &vec2[1];
    // TODO start
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i += 10;
    }

    for i in &vec2 {
        println!("{}", i);
    }
    // TODO end
    println!("Vec Length {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());

    //----------------------------------------------------------------------------------
    // function
    say_hello();
    get_sum(2, 3);
    println!("{}", get_sum(3, 4));
    println!("{}", get_sum2(5, 4));
    let (val_1, val_2) = get_2(6);
    println!("Nums: {}, {}", val_1, val_2);

    let num_list = vec![1, 2, 3, 4, 5, 6];
    println!("Sum of list = {}", sum_list(&num_list));

    //----------------------------------------------------------------------------------
    // generic
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.8 = {}", get_sum_gen(5.2, 4.8));
    
    //----------------------------------------------------------------------------------
    // Ownership
    // stack: LIFO with a defined fixed size
    // heap: When putting data on the heap you request a certain amount of space. The OS finds
    // space available and returns an address for that space called a pointer.
    // eg.
    let mut my_int: i32 = 1;    // The variable and value are stored on a stack
    let mut my_vec: Vec<i32> = vec![1,2,3];  // The variable and pointer are stored on a stack, the pointer points to the heap where the values are stored
    drop(my_vec);   // both the stack and heap will no longer be accessible
    // RULES: 
    //  1. Each value has a variable called its owner
    //  2. There is only one owner at a time
    //  3. When the owner goes out of scope the value will be dropped
    let str1 = String::from("World");   // str1 is the owner of value(heap)
    let str2 = str1;    // Transfer the ownership from str1 to str2(we lost str1)
    // println!("Hello {}", str1); // you are going to see an error here
    let str3 = String::from("World");
    let str4 = str3.clone();
    println!("Hello {}", str3); //no error
    print_str(str3);
    let str5 = print_return_str(str4);
    println!("str5 = {}", str5);
    let mut str6 = String::from("new world");
    change_string(&mut str6);


}

//----------------------------------------------------------------------------------
// function
fn say_hello() {
    println!("Hello World!");
}

fn get_sum(x: i32, y: i32) -> i32 {
    println!("{} + {} = {}", x, y, x + y);
    x + y // no ";", statement doesn't evaluate to function?
}

fn get_sum2(x: i32, y: i32) -> i32 {
    println!("{} + {} = {}", x, y, x + y);
    return x + y; // or use return
}

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(my_list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in my_list.iter() {
        sum += &val;
    }
    sum
}

//----------------------------------------------------------------------------------
// generic

use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    // it doesn't have be "T" here, can be anything
    return x + y;
}
    //----------------------------------------------------------------------------------
    // Ownership
fn print_str(x: String) {
    println!("A string {}", x);
}
fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}
fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("msg: {}", name);

}
