fn main() {
    // Ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        // let s = "hello"; // s is on stack
        let _s = String::from("hello"); // s is on heap    
    } 

    let x = 5;
    let _y = x; // copy

    let s1 = String::from("hello");
    // let s2 = s1; // move (not shallow copy)
    let _s2 = s1.clone(); // deep copy
    println!("{}, world!", s1); // error in shallow copy

    // Ownership and functions
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);
    // println!("{}", s); // error

    let x1=5;
    makes_copy(x1);
    println!("{}", x1); // ok because i32 is Copy

    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    println!("{}", s1);

    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    println!("s1 = {}, s3= {}", s1, s3);

    // Refrences when you don't want to move
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // change value of mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let mut st = String::from("hello");
    let r1 = &st;
    let r2 = &st;
    println!("{}, {}", r1, r2);

    let r3 = &mut st; // if r1 and r2 are not used after this, this will be ok
    println!("{}", r3);

    // dangling reference
    // let reference_to_nothing = dangle(); // error

    // slices
    // let s = String::from("hello world");
    // let hello = &s[..5];
    // let world = &s[6..];
    // println!("{}, {}", hello, world);
    let word = first_word(&s);
    println!("first word is {}", word);

    // slice on different types of collections
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    println!("{:?}", slice);


}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                     // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn calculate_length(s: &String) -> usize { 
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s (invalid memory reference)
}
*/

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}