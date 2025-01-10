use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    print!("Guess the number \n");
    println!("please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    let guess: u32 = guess.trim().parse().expect("please type a number");


    let random_num = rand::thread_rng().gen_range(1..=100);

    print!("{}" , random_num);

    match guess.cmp(&random_num) {
        Ordering::Less => println!("much bigger than the generated num"),
        Ordering::Greater => println!("much bigger than the generated num"),
        Ordering::Equal => println!("much bigger than the generated num"),
    }

}