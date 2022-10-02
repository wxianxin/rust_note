// 20221002
#![allow(unused)] // suppress warnings for unused variables

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!"); // println is macro (with "!")
                               // Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
                               // Macros are executed at compile time. They generally expand into new pieces of code that the compiler will then need to further process.
    println!("What's your name?");
    // Declare variables: Variables are bound to values using 'let'
    let mut name = String::new(); // in rust by default the variables defined are immutable
    let greeting = "Nice to meet you.";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input.");
    println!("Hello {}! {}", name.trim_end(), greeting);

    //----------------------------------------------------------------------------------

    const ONE_BIL: u32 = 1_000_000_000; // naming convention: all cap with underscore
    const PI: f32 = 3.1415926;
    let age = "30";
    // shadowing: define variables with the same name
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_BIL);
    // shadowing: define variables with the same name but different types

    // scalar datatypes
    /*  integer
        Length	Signed	Unsigned
        8-bit	i8	u8
        16-bit	i16	u16
        32-bit	i32	u32
        64-bit	i64	u64
        128-bit	i128	u128
        arch	isize	usize       // using this will assign the length of computer architecture
                                    // integer defaults to i32, which is generally the fastest
                                    // float defaults to f64, for modern cpu, as fast as f32
    */

    println!("Max u32: {}", u32::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);

    let is_true: bool = true; // boolean is of 1 byte in size
    let my_grade: char = 'A'; // character is of 4 bytes in size, representing unicode

    //----------------------------------------------------------------------------------
    // math
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);

    let random_num = rand::thread_rng().gen_range(1..101); // [1, 101) -> [1, 100]
    println!("random integer from 1 - 100: {}", random_num);

    //----------------------------------------------------------------------------------
    // control flow
    let age: i32 = 30;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday")
    } else if (age == 21) {
        println!("Important Birthday")
    } else {
        println!("Not an Important Birthday")
    }

    // ternary operator
    let mut my_age = 18;
    let can_vote = if my_age >= 18 { true } else { false }; // no ':' after 'true' because this is
                                                            // an assignment instead of statement
    println!("Can Vote: {}", can_vote);

    // match; use scrutinee expression (use a value to compare to the patterns) each arm has a pattern and some code. use "=>" to separate
    // "match" is functionally similar to "if", the big difference is that if needs to return boolean but match can return any datatype
    let age2 = 9;
    match age2 {
        // the scrutinee expression is "age2"
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
    // tuple: contain multiple datatypes in a list of a fixed size
    let my_tuple: (u8, String, f64) = (32, "Steven".to_string(), 10_0000.00);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
    let empty_tuple = (); // empty tuple is called "unit"

    //----------------------------------------------------------------------------------
    // loop: loops forever
    let mut i = 1;
    let sth = loop {
        i *= 2;
        if i > 100 {
            break i; // return from loop, put the value after "break"
        }
    };
    assert_eq!(sth, 128);

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

    // for; best for iterate over elements in a collection
    for x in arr_1.iter() {
        println!("Val: {}", x)
    }

    //----------------------------------------------------------------------------------
    // string: UTF-8, Non-Null-Byte Terminated, Not collections of chars
    // 2 types of string:
    //      String -> called owned string, owns string data, data freed when dropped, vector of bytes that can be changed
    //          3 parts of a String: 1. length 2. Capacity (memory already reserved) 3.Data pointer
    //      &str -> a borrowed string slice, does not own string data, data not freed when dropped, point to the string and allow for viewing
    //      2 parts of &str: 1. length 2. Data pointer
    // string literals: Embedeed into the compile binary, have type &str

    let mut s = String::from("a few \n lines");
    let mut sl = "hello\nworld!"; // This is a string slice from a string literal
    let upper = s.to_uppercase();
    let stripped = upper.strip_prefix("HELLO");
    println!("-----------------------------------{:?}", stripped);

    println!("{}", s.lines().next().unwrap());
    let new_s = s.lines().next().unwrap();
    let new_s = &s.lines().next().unwrap().to_owned();

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
    /* collections
        Vec<T>          // Access by index
                        // 3 parts: 1. length 2. capacity 3. pointer to the data
        HashMap<K,V>    // Associate keys and values
        HashSet<T>      // Unique items & no access by index
        VecDeque<T>     // efficiently add/remove from both the front and back(Vec is only about the end); Good for a queue
        LinkedList<T>   // less often used in Rust
    */
    let v = vec![1u8, 2, 3];
    let s = &v[0..2]; // a slice is continuous chunks of memory
                      // 2 parts: 1. length 2. data pointer
                      //----------------------------------------------------------------------------------
                      // casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    //----------------------------------------------------------------------------------
    // enum: similar to structs, but with more flexibility and advantages
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    let quit = WebEvent::KeyPress('q'); // use enum as datatype

    /*
    option enum, defined in std library
    NOTE Rust does not have the concept of null values, but it has enum to encode whether a value being null or not null
    enum Option<T> {    // "T" is a generic assocaite with some type
        Some(T),
        None,
    }
    */
    let soemthing = Some(1); // do not need to bring into scope intentionally

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

    let today: Day = Day::Friday; // use enum as datatype
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
    // if function has no return, then the return type is unit
    // "return" is optional, last value is returned
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
    let mut my_int: i32 = 1; // The variable and value are stored on a stack
    let mut my_vec: Vec<i32> = vec![1, 2, 3]; // The variable and pointer are stored on a stack, the pointer points to the heap where the values are stored
    drop(my_vec); // both the stack and heap will no longer be accessible
                  // RULES: NOTE
                  //  1. Each value has a variable called its owner
                  //  2. There is only one owner at a time
                  //  3. When the owner goes out of scope the value will be dropped
    let str1 = String::from("World"); // str1 is the owner of value(heap)
    let str2 = str1; // Transfer the ownership from str1 to str2(we lost str1)
                     // println!("Hello {}", str1); // you are going to see an error here
    let str3 = String::from("World");
    let str4 = str3.clone();
    println!("Hello {}", str3); //no error
    print_str(str3);
    // println!("{}", str3);    // NOTE this will fail because the ownership has been
    // transfered to the function print_str, so in regular
    // main scope str3 is no longer valid
    //----------------------------------------------------------------------------------
    // borrowing
    let str5 = String::from("World");
    print_reference_str(&str5);
    // println!("str4 = {}", str4);
    println!("str5 = {}", str5); // now it works
    let mut str6 = String::from("new world");
    change_string(&mut str6);

    // references, like variables, are immutable by default
    // RULES: NOTE
    //  1. At any given time: One mutable reference OR any number of immutable references
    //  2. References must always be valid
    let mut my_vec = vec![1, 2, 3, 4];
    println!("{:?}", my_vec);
    add_to_vec(&mut my_vec);
    println!("{:?}", my_vec); // mutate the original vector

    let say = String::from("dog");
    let say2 = &say;
    println!("{}", say);
    println!("{} {}", say, say2);
    drop(say);
    // println!("{}", say2);    // this would fail, (say2)'s lifetime has to be within (say)'s lifetime

    //----------------------------------------------------------------------------------
    // hashmap
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");

    let superman = heroes.get("Superman"); // superman is a reference
    let batman = heroes.remove("Batman"); // batman is the owner of the value

    for (k, v) in heroes.iter() {
        println!("{} = {}, total_length: {}", k, v, heroes.len());
    }

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

    //----------------------------------------------------------------------------------
    // struct

    struct Point2D(u32, u32); // tuple struct
    let origin = Point2D(100, 200);
    println!("Point contains {:?} and {:?}", origin.0, origin.1);

    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob: Customer = Customer {
        name: String::from("Bob"),
        address: String::from("111 Main St."),
        balance: 123.50,
    };

    bob.address = String::from("345 Main St.");

    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle {
        length: 4,
        height: 5,
    };
    //----------------------------------------------------------------------------------
    // trait
    // Unlike trading OOP language, where data and behavior(methods) are within an object
    // In Rust, data is kept in Enum/Struct, bahavior is kept separately in Trait.
    // In this way, you can mix and match the data and trait
    let person = Person{name: String::from("Ss")};
    let cat = Cat{name: String::from("Cc")};
    let rabbit = Rabbit{name: String::from("Rr")};
    person.eat_dinner();
    cat.eat_dinner();
    rabbit.eat_dinner();

    //----------------------------------------------------------------------------------
    // modules TODO skipped
    // Crates : Modules that produce a library or executable
    // Modules : Organize and handle privacy
    // Packages : Build, test and share crates
    // Paths : A way of naming an item such as a struct, function

    //----------------------------------------------------------------------------------
    // error handling
    // panic!("terrible error!");   // use panic only when the program comes into an unrecoverable state
    //
    /* Option enum
    enum Option<T> {    // "T" is a generic assocaite with some type
        Some(T),
        None,
    }
    */

    /* Result num, principally used for input/output operation where failture is expected
    enum Result<T, E> { // both T,E are generics
        Ok(T),  // represents success and contains a value
        Err(E), // represents an error
    }
    */

    /*
    // helper method to avoid explicitly typing in "match" statements
    // unwrap() returns the value inside the Ok variant. Returns a panic! macro for the Err variant.
    // Ok
    File::open("hello.txt").unwrap();
    // expect() returns the value or call the panic! macro with a detailed error message.
    // Err
    File::open("hello.txt").expect("Failed to open hello.txt");
    */

    /* The ? operator: like a match statement
     * For Result type: Unwraps the value of "Ok" variant; Returns an error if "Err" variant
     * For Option type: Returns a value is with the "Some" variant; Returns nothing for the "None" variant
     */

    //----------------------------------------------------------------------------------
    // File IO
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };

    write!(output, "Just some\nRandom words.").expect("Facild to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };

    //----------------------------------------------------------------------------------
    // iterator
    // An iterator cycles through values by borrowing, so the collection is not moved (You can't change values)
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    // And call for each value with next
    println!("1st : {:?}", iter1.next());

    //----------------------------------------------------------------------------------
    // closure
    // A closure is a function without a name and they are sometimes stored in a variable (They can be used to pass a function into another function)
    // let var_name = |parameters| -> return_type {BODY}
    let can_vote = |age: i32| age >= 18;
    println!("Can vote : {}", can_vote(8));

    // Closures can access variables outside of its body with borrowing
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 10;

    // You can change values if you mark the closure mutable
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);

    // You can pass closures to functions
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    // you can define function within main
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

    //----------------------------------------------------------------------------------
    // SMART POINTERS
    // A pointer is an address to a location in memory. We have been
    // using them when we used the reference operator(&) to borrow
    // a value.

    // Strings and vectors are smart pointers. They own
    // data and also have functions for manipulating that data.

    // Smart pointers provide functionality beyond referencing locations
    // in memory. They can be used to track who has ownership of data.
    // Ownership is very important with Rust.

    //----------------------------------------------------------------------------------
    // BOX

    // The Box smart pointer stores data on the heap instead of the stack.
    // All values are stored on the stack by default

    // A Box is normally used when you have a large amount of data stored
    // on the heap and then you pass pointers to it on the stack.
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    // TODO start
    // If we try to create a Binary tree we get the error
    // the size for values of type `str` cannot be known at
    // compilation time within `TreeNode<T>`

    // This is saying we can't include nodes in a node because
    // the size of node depends on the size of multiple nodes
    // which confuses the compiler
    // struct TreeNode<T> {
    //     pub left: TreeNode<T>,
    //     pub right: TreeNode<T>,
    //     pub key: T,
    // }

    // We have other problems in that Binary Trees eventually end
    // and Rust doesn't like Null values so we have to use Option

    // We can use a Box here because it has a pointer to data and
    // a fixed size
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    // Create functions for creating nodes and adding left & right
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    // Create the root node with left and right
    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));

    //----------------------------------------------------------------------------------
    // concurrency
    // Common problems with parallel programming involve :
    // 1. Thread are accessing data in the wrong order
    // 2. Threads are blocked from executing because of confusion
    // over requirements to proceed with execution

    let thread1 = thread::spawn(|| {
        // create a thread
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // There are no guarantees on when the threads will execute and wether they will complete execution
    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // We call join here so that the main thread executes with thread1
    thread1.join().unwrap(); // unwrap handles the option Result which is Ok or Err
                             // TODO .get() .unwrap()
                             // TODO &mut

    // TODO start
    pub struct Bank {
        balance: f32,
    }

    use std::cell::RefCell;
    use std::rc::Rc;
    // Arc<T> provides shared ownership of a value
    // Mutex blocks threads waiting for lock to be available
    use std::sync::{Arc, Mutex};

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance : {} Withdrawal a smaller amount",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdrew {} Current Balance {}",
                amt, bank_ref.balance
            );
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));

    // Creates 10 customer threads
    let handles = (0..10).map(|_| {
        // Clone duplicates an the bank object
        let bank_ref = bank.clone();
        thread::spawn(|| customer(bank_ref))
    });

    // Wait for all customers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total: {}", bank.lock().unwrap().balance);
    // TODO end
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
fn print_reference_str(x: &String) {
    println!("A reference string {}", x);
}
fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("msg: {}", name);
}
fn add_to_vec(my_vec: &mut Vec<i32>) {
    my_vec.push(4);
}
//----------------------------------------------------------------------------------
// Trait

pub struct Person {
    name: String
}
pub struct Cat {
    name: String
}
pub struct Rabbit {
    name: String
}
pub trait Eat {
    fn eat_dinner(&self) {
        println!("I eat dinner")
    }
}
impl Eat for Person {
    fn eat_dinner(&self) {
        println!("I eat from a plate")
    }
}
impl Eat for Cat {
    fn eat_dinner(&self) {
        println!("I eat from a cat bowl")
    }
}
impl Eat for Rabbit {}
