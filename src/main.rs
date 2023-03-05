use std::io;
//use std::io::{Write,BufReader, BufRead, ErrorKind};
//use rand::Rng;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you!";

    io::stdin().read_line(&mut name)
        .expect("Did'nt receive Input!");

    println!("Hello {}! {}", name.trim_end(), greeting);
}
