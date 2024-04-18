// 20221002
#![allow(unused)] // suppress warnings for unused variables

use rand::Rng;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!"); // println is macro (with "!")
                               // Fundamentally, macros are a way of writing code that
                               // writes other code, which is known as metaprogramming.
                               // Macros are executed at compile time. They generally
                               // expand into new pieces of code that the compiler will
                               // then need to further process.
    println!("{}", "Hello there!"); // same as the following block

    use std::io::{self, Write}; // same as above
    io::stdout().lock().write_all(b"Hello there!\n").unwrap();                                

    const ONE_BIL: u32 = 1_000_000_000; // naming convention: all cap with underscore
    const PI: f32 = 3.1415926;
    let age = "30";
    // shadowing: define variable with the same name, essentially reassign the variable
    // name to a new variable.
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
    // logic operator
    // &&   // logical AND
    // ||   // logical OR
    // !    // logical NOT
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
    let can_vote = if my_age >= 18 { true } else { false }; // no ':' after 'true'
                                                            // because this is an
                                                            // assignment instead of
                                                            // statement
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
        std::cmp::Ordering::Less => println!("Can't Vote"),
        std::cmp::Ordering::Greater => println!("Can Vote"),
        std::cmp::Ordering::Equal => println!("You gained the right to vote."),
    };

    //----------------------------------------------------------------------------------
    // array, elements in array must be of the same datatype, array has fixed size
    let arr_1: [i32; 4] = [1, 2, 3, 4];
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st: {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

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
            break i; // return i from loop (put the value after `break`)
        }
    };
    // Rust loops don't have a return value unless you're using a loop expression (e.g., `let val = loop { ... }`).
    assert_eq!(sth, 128);

    let mut loop_idx = 0;
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
    // Rust's String is a wrapper over a `Vec<u8>`.
    // string: UTF-8, Non-Null-Byte Terminated, Not collections of chars
    // 2 types of string:
    //      String -> called owned string, owns string data, data freed when dropped, vector of bytes that can be changed. 3 parts of a String: 1. length 2.Data pointer 3. Capacity (memory already reserved)
    //      &str -> a borrowed string slice, does not own string data, data not freed when dropped, point to the string and allow for viewing
    //      2 parts of &str: 1. length 2. Data pointer
    // string literals: Embedeed into the compile binary, have type &str

    let mut s = String::from("a few \n lines");
    let s = "a few \n lines"; // "a few \n lines" here is static and immutable
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

    //---------------------------------------------------------------------------------
    // stdin
    println!("What's your name?");
    // Declare variables: Variables are bound to values using 'let'
    let mut name = String::new(); // in rust by default the variables defined are immutable
    let greeting = "Nice to meet you.";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input.");
    println!("Hello {}! {}", name.trim_end(), greeting);

    //----------------------------------------------------------------------------------
    //----------------------------------------------------------------------------------
    /* collections
        Vec<T>          // Access by index
                        // 3 parts: 1. length 2. capacity 3. pointer to the data
        std::collections::HashMap<K,V>    // Associate keys and values
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
    // struct vs. enum
    // think struct as a new type that you defined
    //  Use a struct when you want to aggregate several values together.
    //  Use an enum when you want to describe a type that can be one of a few different kinds of values.
    //----------------------------------------------------------------------------------
    // enum or enumeration is a data type that allows you to define a type by enumerating its possible variants. Each variant can optionally have data associated with it.
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
    The Option enum in Rust is a way of handling optional values without having to rely on error-prone conventions like returning a special value (such as -1 or null). It's a safer way of dealing with the absence of a value, which is a common occurrence in programming.
    Option is an enum that has two variants: Some and None.
        Some(T): An element of type T is present
        None: No element is present
    NOTE Rust does not have the concept of null values, but it has enum to encode whether a value being null or not null
    enum Option<T> {    // "T" is a generic assocaite with some type
        Some(T),
        None,
    }
    */
    let soemthing = Some(1); // do not need to bring into scope intentionally

    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;

    // In this example, Direction is an enum with four variants. You can use the :: syntax to specify a particular variant of the enum.
    match dir {
        Direction::North => println!("We are heading north!"),
        Direction::South => println!("We are heading south!"),
        Direction::East => println!("We are heading east!"),
        Direction::West => println!("We are heading west!"),
    }

    // example 2
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Rectangle(width, height) => width * height,
            }
        }
    }

    let circle = Shape::Circle(10.0);
    println!("The area of the circle is {}", circle.area());

    let rectangle = Shape::Rectangle(10.0, 5.0);
    println!("The area of the rectangle is {}", rectangle.area());

    //----------------------------------------------------------------------------------
    // Arrays in Rust have a fixed size, which must be known at compile time.
    // Vectors are similar to arrays, but they can grow and shrink at runtime.
    let vec1: Vec<i32> = Vec::new(); // define a new Vec
    let mut vec2 = vec![1, 2, 3, 4]; // similar to array literals in other languages.
    let v = vec![0; 10]; // This creates a new vector v with ten elements, all of them set to 0.
    let v2 = v.clone(); // create a copy of a vector
    println!("{:?}", vec2); // To display a Vec in Rust, you can simply print it using the println! macro with the {:?} formatter. The {:?} formatter is used for debug formatting, and Vec implements the Debug trait, so it can be printed this way.
    vec2.push(5); // add element to Vec
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
    // In Rust, if a function doesn't have an explicit return statement, it implicitly returns the unit type, (). The unit type in Rust is similar to void in C/C++ or None in Python, it essentially represents "nothing" or "no value".
    // "return" is optional, last value is returned.
    // You can define functions before or after main
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
    // heap: When putting data on the heap you request a certain amount of space. The OS
    // finds space available and returns an address for that space called a pointer.
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
    // There are two kinds of borrows: mutable and immutable.
    // An immutable reference (&T) allows you to borrow a value for reading, but not for writing. You can have multiple immutable references to a value at the same time.
    // A mutable reference (&mut T) allows you to borrow a value for reading and writing. You can only have one mutable reference to a specific value in a particular scope.
    // Rust enforces a rule that says you cannot have a mutable reference while you have an immutable one, because users of an immutable reference don't expect the values to suddenly change out from under them!
    // In Rust, borrowing and referencing are closely related concepts. They are different aspects of the same feature of the language.
    // References: The mechanism for pointing to a value without taking ownership. You create a reference by using the & symbol. A reference doesn't own the value it points to. There are two types of references in Rust, &T and &mut T, where T is any type. The first one, &T, is an immutable reference, meaning you cannot change the value through that reference. The second one, &mut T, is a mutable reference, which lets you change the value it points to.
    // Borrowing: This is the act of taking a reference to a value. When you take a reference, you are "borrowing" the value. You can either take an immutable borrow, or a mutable borrow. Borrowing allows you to use a value in different parts of your code without transferring ownership. This is important in Rust because the language enforces strict ownership and borrowing rules at compile time to prevent issues like use-after-free, null pointer dereferencing, and data races.
    let str5 = String::from("World");
    print_reference_str(&str5);
    // println!("str4 = {}", str4);
    println!("str5 = {}", str5); // NOTE: now it works
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
    let mut heroes = std::collections::HashMap::new();
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
        // The T and U in the definition are placeholders for
        // generic types.
        length: T,
        height: U,
    }
    let rec = Rectangle {
        length: 4,
        height: 5.2,
    };
    //----------------------------------------------------------------------------------
    // trait
    // Unlike traditional OOP language, where data and behavior(methods) are defined within an object
    // In Rust, data is kept in Struct/Enum, bahavior is kept separately in Trait.
    // In this way, you can mix and match the data and trait
    // Rust encourages a data-oriented design and the use of composition over inheritance.
    let person = Person {
        name: String::from("Ss"),
    };
    let cat = Cat {
        name: String::from("Cc"),
    };
    let rabbit = Rabbit {
        name: String::from("Rr"),
    };
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
     * Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it.
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
    // An iterator cycles through values by borrowing, so the collection is not moved
    // (You can't change values)
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    // And call for each value with next
    println!("1st : {:?}", iter1.next());

    fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
        map.iter().fold(0, |acc, (_, v)| { if v == &value {acc + 1} else {acc}})
    }

    //----------------------------------------------------------------------------------
    // closure
    // A closure is anonymous function and it is sometimes stored in a
    // variable (They can be used to pass a function into another function)
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
    fn use_func<T>(
        a: i32,
        b: i32,
        func: T,
    ) -> i32
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
    // The main difference between normal pointers and smart pointers is that smart pointers have additional functionalities, like automatic cleanup (deallocation of heap memory) or dereferencing capabilities.
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
    // The Box<T> is the simplest form of smart pointer. It allows you to store data on the heap rather than the stack, and the box will be deallocated when it goes out of scope, freeing the memory.
    // All values are stored on the stack by default

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

        pub fn left(
            mut self,
            node: TreeNode<T>,
        ) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(
            mut self,
            node: TreeNode<T>,
        ) -> Self {
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

    fn withdraw(
        the_bank: &Arc<Mutex<Bank>>,
        amt: f32,
    ) {
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

fn get_sum(
    x: i32,
    y: i32,
) -> i32 {
    println!("{} + {} = {}", x, y, x + y);
    x + y // In Rust, if you omit the semicolon at the end of the last expression in a function, that expression is treated as the return value. This is a feature of "expression-based" languages like Rust
}

fn get_sum2(
    x: i32,
    y: i32,
) -> i32 {
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
// In Rust, generics are abstract stand-ins for concrete types or other properties. They allow for writing code that is flexible and avoids duplication, yet still type safe.
// For example, you might want a function to work on different types of data. Instead of writing separate functions for each type, you can write a single function that takes a generic type parameter.

use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>>(
    x: T,
    y: T,
) -> T {
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
    name: String,
}
pub struct Cat {
    name: String,
}
pub struct Rabbit {
    name: String,
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

// 20231216
// https://fasterthanli.me/articles/a-half-hour-to-learn-rust
// expression will be evaluated to a value
// Blocks "{}" are also expressions
// if clause is an expression
// match is an expression, not a statement
let least = std::cmp::min(3, 8) // In this example, std is a crate (~ a library), cmp is a module (~ a source file), and min is a function
// use directives can be used to "bring in scope" names from other namespace
// this works:
use std::cmp::min;
use std::cmp::max;
//Within use directives, curly brackets have another meaning: they're "globs". 
// this also works:
use std::cmp::{min, max};
// this also works!
use std::{cmp::min, cmp::max};
// Types are namespaces too, and methods can be called as regular functions
// vec uses heap allocated array, in run time when the array runs out, it swaps for a larger array
// when it reaches full capacity

// tuple
let pair = ('a', 17);
pair.0; // this is 'a'
pair.1; // this is 17

// There is a shortcut for initializing the rest of the fields from another struct
// struct update syntax: can only happen in last position, and cannot be followed by a comma.
let v3 = Vec2 {
    x: 14.0,
    ..v2
};
// Traits are something multiple types can have in common
// Definition: A trait in Rust is a collection of methods defined for an unknown type: Self. Traits can include readable and writable properties, methods, and other traits.
// Implementations: Traits are implemented by data types (like structs or enums). A single type can implement many traits, and a trait can be implemented by many types.
trait Signed {
    fn is_strictly_negative(self) -> bool;
}

//You can implement:
//
//    one of your traits on anyone's type
//    anyone's trait on one of your types
//    but not a foreign trait on a foreign type

// In Rust, a type refers to the kind of data that a value can hold. This includes primitive types (like i32, f64, bool), compound types (like tuples), user-defined types (like structs and enums), and more complex types (like closures and function pointers).
// Rust prefers composition over inheritance and uses traits to share behavior between different structs.
// An impl block is always for a type, so, inside that block, `Self` means that type
//
// if is an expression

// if let expressions mainly as a shorter way to write the equivalent of a match that only matches one case
// you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
// example:
if let Some(3) = some_u8_value {
    println!("three");
}

// chapter 4
// stack: LIFO, All data stored on the stack must have a known, fixed size. 
// All data stored on the stack must have a known, fixed size.
// heap: The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack
// Pushing to the stack is faster than allocating on the heap because the allocator never has to
// search for a place to store new data; that location is always at the top of the stack.

// 4.2 References and Borrowing
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
// We also cannot have a mutable reference while we have an immutable one to the same value.
// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

// 4.3 The Slice Type 
// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

// 5.2 
// println!("rect1 is {:?}", rect1);. Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug
// `{:?}` (or {:#?} for pretty-print)
#[derive(Debug)] // Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struc
struct Rectangle {
    width: u32,
    height: u32,
}
// Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
// Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout).
// 5.3
// We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
// The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization.
// automatic referencing and dereferencing, the following are the same:
// We can define associated functions that don’t have self as their first parameter (and thus are not methods)
p1.distance(&p2);
(&p1).distance(&p2);
// We can define associated functions that don’t have self as their first parameter (and thus are not methods)


// 7.1
// A package can contain as many binary crates as you like, but at most only one library crate.
// If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

// 7.3
// Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.
//
// https://rust-lang.github.io/api-guidelines/
// If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.
// In contrast, if we make an enum public, all of its variants are then public.
// Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public.

// 8
// https://doc.rust-lang.org/std/collections/index.html
// 8.1
// https://doc.rust-lang.org/std/vec/struct.Vec.html

// 8.2
// String is actually implemented as a wrapper around a vector of bytes with some extra guarantees

// 9.1
// Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here.
// 9.2
// We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
// A Shortcut for Propagating Errors: the ? Operator after a Result value (Return early Err/None)
// The error message also mentioned that ? can be used with Option<T> values as well. To deal with `None`
// Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.

// 9.3
// returning Result is a good default choice when you’re defining a function that might fail.
// unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors
// However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call.

// 10.3
// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

// 11.1
// Attributes are metadata about pieces of Rust code. To change a function into a test function, add #[test] on the line before fn.
// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
// 
// adder
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     └── integration_test.rs
// 
// cargo test -- --test-threads=1
// cargo test -- --show-output
// cargo test test_name
// cargo test -- --ignored //  If we want to run only the ignored tests
// cargo test -- --include-ignored



// 12.1
// cargo run -- searchstring example-filename.txt // two hyphens to indicate the following arguments are for our program rather than for cargo
// 12.4
// (Note that the backslash after the opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal)

// 13.2
// Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.
// Also note that the values we get from the calls to next are immutable references to the values in the vector. The iter method produces an iterator over immutable references. If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.
// Methods that call next are called consuming adaptors, because calling them uses up the iterator.
// Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.

// 15
// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
// while references only borrow data, in many cases, smart pointers own the data they point to.
// Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers. The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.
// 15.3
// Variables are dropped in the reverse order of their creation
// 15.6
// This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default with regular references. The Box<T> type has a known size and points to data allocated on the heap. The Rc<T> type keeps track of the number of references to data on the heap so that data can have multiple owners. The RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.
// Also discussed were the Deref and Drop traits, which enable a lot of the functionality of smart pointers

// 16.1
// We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of thread::spawn in a variable. The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.
// Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
// We'll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another
// 16.2
// Using Message Passing to Transfer Data Between Threads
// A channel has two halves: a transmitter and a receiver. A channel is said to be closed if either the transmitter or receiver half is dropped.
// 16.4
// The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads.
// The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads. In other words, any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread.
