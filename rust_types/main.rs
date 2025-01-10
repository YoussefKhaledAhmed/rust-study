use std::any::type_name;
/**
 * Function to represent the idea of the 
 * immutable variables.
 */
fn immutable_var() {
    /* here it is immutable variable  *
     * where it can't be changed like *
     * const vaiables in C.           */
    let x = 5;
    println!("immutable variables function");
    println!("x -> {x}");

    // x = 6; @error -> as cannot assign twice to immutable variable `x`
    // println!("x -> {x}");
}

/**
 * Function to represent the idea of the 
 * mutable variables.
 */
fn mutable_var() {
    /* here it is immutable variable  *
     * where it can't be changed like *
     * const vaiables in C.           */
    let mut x = 5;
    println!("mutable variables function");
    println!("x -> {x}");
    x = 6; 
    println!("x -> {x}");
}

/**
 * Function to represent the idea of the 
 * mutable variables.
 */
fn const_var() {
    /* constant variables: */
    /* specifying the type is mandatory      *
       const TIME_PER_3_HRS = 60*3; -> ERROR */ 
    const TIME_PER_3_HRS:u32 = 60*3;
    println!("const variable: ");
    println!("time in 3 hours: {TIME_PER_3_HRS}");
    // TIME_PER_3_HRS = 3; @error -> Can't assign to this expression
    // println!("time in 3 hours: {TIME_PER_3_HRS}");
}

/**
 * Function to print type name
 */
fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

/**
 * Function to present the shadowing idea of variables in 
 * rust.
 */
fn var_shadowing(){
    
    println!("shadowing in RUST");
    let spaces = "";
    println!("spaces type before shadowing: {}" , type_of(&spaces));
    let spaces = 32;
    println!("spaces type after shadowing: {}" , type_of(&spaces));
}

/**
 * Function to describe the tuple data type
 */
fn tuple_var(){
    let tup = (1,"str",3.6);
    let (x,y,z) = tup;
    println!("tuple values using destructuring: {} , {} , {}" , x , y , z);
    
    println!("tuple values using indecies: {} , {} , {}" , tup.0 , tup.1 , tup.2);
}

/**
 * Function to describe arrays in RUST
 */
fn array_var(){
    println!("array handling: ");
    /* normal declaration */
    let mut arr_1 = [1,2,3];
    println!("printing arr_1");
    print_array(&mut arr_1);

    let mut arr_2:[i32;3] = [4,5,6];
    println!("printing arr_2");
    print_array(&mut arr_2);

    let mut arr_3 = [9;3];
    println!("printing arr_3");
    print_array(&mut arr_3);
}

/**
 * Function to print an array
 */
fn print_array(arr: &mut [i32]){
    for i in 0..arr.len(){
        println!("arr[{}] -> {}" , i , arr[i]);
    }
}

fn main(){
    /* testing immutable variables */
    immutable_var();

    /* testing mutable variables */
    mutable_var();

    /* Const variables */
    const_var();

    /* variables shadowing */
    var_shadowing();

    /* testing tuples */
    tuple_var();

    /* testing arrays */
    array_var();

}