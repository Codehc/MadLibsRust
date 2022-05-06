use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn mainLol() {
    println!("Welcome to RNGuesser!");

    println!("Pick a minumum value:");
    let min: i32 = get_num();

    let mut max: i32;
    loop {
        println!("Pick a maximum value:");
        max = get_num();
        if max > min {
            break;
        }
        /*match max.cmp(&min) {
            Ordering::Less => (),
            Ordering::Greater => break,
            Ordering::Equal => (),
        }*/
        println!("{} is too small. Maximum value must be larger than the minimum value ({}).", max, min);
    }
    let max = max;

    println!("\nRange: {}..{}", min, max);

    let secret = rand::thread_rng().gen_range(min..=max);

    loop {
        println!("\nPick a number... any number!");
        let guess = get_num();

        match guess.cmp(&secret) {
            Ordering::Less => println!("{} is too small.", guess),
            Ordering::Greater => println!("{} is too big.", guess),
            Ordering::Equal => {
                println!("{} is right on the money!", guess);
                break;
            },
        }
    }
}

fn get_num() -> i32 {
    loop {
        let mut string = String::new();
        io::stdin().read_line(&mut string).expect("Please enter a valid string.");

        let parsed: i32 = match string.trim().parse() {
            Ok(result) => result,
            Err(_) => continue,
        };

        return parsed;
    }
}