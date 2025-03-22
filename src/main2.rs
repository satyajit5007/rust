// fn main() {
//     println!("Hello, world!");
//     println!("removing the old and adding the new");
//     // start new coding => on the ownership and the memory management
//     // ownership is a unique property of Rust

//     // ownership rules
//     // 1. Each value in Rust has a variable that owns it
//     // 2. There can only be one owner at a time
//     // 3. When the owner goes out of scope, the value will be dropped

//     // ownership transfer
//     // 1. Move
//     // 2. Borrow
//     // 3. Clone

//     //borrowing and references

//     // owner =>user1
//     // owner =>user1 -> borrow1 , borrow2 , borrow3


//     //example1
//     let s1 = String::from("Hello");
//     let s2 = &s1; //valid these because of the '&

//     println!("{}", s2);
//     println!("{}", s1); // This is valid s1, The first pointer wasn't invalidated



// }
//-----------------------------------------------------------------------------------------------------------------
//example 2 borrowing and multiple borrowing

// fn main() {
//     let my_string = String::from("Hello, Rust!");
//     let s1 = &my_string;  //Pass a reference to my_string
//     let s2 = &my_string;
//     let s3: &String = &my_string;

//     println!("{}", s1);    // This is valid because ownership was not transferred
//     println!("{}", s2);
//     println!("{}", s3); //multiple borrowing
//     println!("{}", my_string);
//     println!("{}", s1);
//     println!("{}", s2);
//     println!("{}", s3); 
// }

// fn takes_ownership(some_string: &String) {
//     println!("{}", some_string);  // some_string is borrowed and not moved
// }


//------------------------------------------------------------------------------------------------
//Mutable references

// fn main() {
//     let mut str: String = String::from("Hello");
//     update_str(&mut str);
//     println!("{}", str);
// }

// fn update_str(str: &mut String){
//     str.push_str(" world");
// }

// fn main(){
//     let mut s1: String = String::from("hello");
//     let s2: &mut String = &mut s1;
//     println!("{}", s2);
//     println!("{}", s1);

// }

// fn main() {
//     let my_string = String::from("Hello, Rust!");
//     takes_ownership(&my_string);  // Pass a reference to my_string
//     println!("{}", my_string);    // This is valid because ownership was not transferred
// }

// fn takes_ownership(some_string: &String) {
//     println!("{}", some_string);  // some_string is borrowed and not moved
// }


fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    update_word(s2);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
