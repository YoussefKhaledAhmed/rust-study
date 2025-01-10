/**
 * Function to take the string
 */
fn take_some_str_value(str: String){
    println!("argument value -> {str}");
}

/**
 * Function to take the string
 */
fn take_some_str_ptr(str: &String){
    println!("argument value -> {str}");
}

/**
 * Function to take the string
 */
fn take_some_variable_value(arg_1: i32){
    println!("argument value -> {arg_1}");
}

/**
 * Function to explain the ownership
 */
fn ownership(){
    let mut str_1 = String::from("hello");
    println!("str1 -> {str_1}");
    let str_2 = str_1.clone();
    println!("str2 -> {str_2}");
    str_1.replace_range(0..1, "H");
    println!("str1 -> {str_1}");
    println!("str2 -> {str_2}");

    /* passing str_1 to take_some_str_value */
    take_some_str_value(str_1);

    /* trying to print str_1 */
    // println!("str_1 -> {str_1}"); -> borrow of moved value

    let x = 5;
    take_some_variable_value(x);

    /* trying to print x */
    println!("x -> {x}"); // no error as x is stored in stack

    /* workaround to prevent the error of variables *
     * saved in the heap.                           */
    /* by passing the address the ownership isn't moved */
    take_some_str_ptr(&str_2);


    /* trying to print str_2 */
    println!("str_2 -> {str_2}");
    
}

/**
 * Function to copy an array into another one
 */
fn copy_array(){
    let arr_1 = [9;3];
    let mut arr_2 = arr_1;

    println!("arr_1[0] -> {}" , arr_1[0]);
    arr_2[0] = 2;
    println!("arr_2[0] -> {}" , arr_2[0]);
}

/**
 * Function to give ownership 
 */
fn gives_ownership() -> String {
    let str = String::from("hello");
    str
}

/**
 * Function to take and giveback ownership
 */
fn take_and_give_back_ownership(str: String) ->String{
    str
}

/** 
 * Function to take a string as an input and
 * returns its ownership and length
 */
fn take_str_and_return_str_length(str: String) -> (String , usize){
    let length = str.len();
    (str,length)
}

/**
 * Function to take a string as a reference but 
 * immutable.
 */
fn calculate_len(str: &String) -> usize{
    str.len()
}

/**
 * Function to take a string as a reference but 
 * mutable.
 */
fn update_str(str: &mut String){
    str.push_str(" , world");
}

/**
 * Function to test using of immutable and mutable 
 * references
 */
fn immutable_mutable_references(){
    let mut s = String::from("youssef");
    let r1 = &s;
    let r2 = &s;
    println!("immutable references -> {r1} , {r2}");

    /*let r3 = &s; this will lead to an error       *
     *             where we can't use consecutive   *
     *             unmutable and mutable references *
     *             without using them.              */
    let r4 = &mut s;
    // println!("mutable reference -> {r3}");
    println!("mutable reference -> {r4}");
}

/**
 * Function to return the index of the last char
 * of the first word
 */
fn first_word(str: String) -> String{
    let bytes = str.as_bytes();
    let mut last_index = 0;
    for (i , &item) in bytes.iter().enumerate() {
        if item == b' '{
            last_index = i;
            break;
        }
    }
    str[..last_index].to_string()
}

fn main(){
    ownership();
    copy_array();
    /* declaring a variable to hold the * 
     * return of the function.          */
    let str:String = gives_ownership();
    println!("given ownership -> {str}");

    /* the new declaration shadows the last one */
    let str:String = take_and_give_back_ownership(str);
    println!("returned ownership -> {str}");

    /* the new declaration shadows the last one *
     * and changed the type of the variable.    */
    let mut str = take_str_and_return_str_length(str);
    println!("the string -> {} , length -> {} ",str.0 , str.1);

    /* passing the immutable string by referece */
    let len = calculate_len(&str.0);
    println!("length of \"{}\" is {len}" , str.0);

    /* passing mutable string by reference */
    update_str(&mut str.0);
    println!("updated string -> {}" , str.0);

    /* testing immutable and mutable references */
    immutable_mutable_references();

    /* getting the first word of a string */
    let scentence = String::from("youssef is embedded sw engineer");
    let first_word = first_word(scentence);
    println!("first word -> {first_word}");
}