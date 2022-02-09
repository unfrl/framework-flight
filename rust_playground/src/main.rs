use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guessing_game(false);
    variables();
    functions();
}

fn guessing_game(should_run: bool) {
    println!("\n~~GUESSING GAME~~\n");

    if !should_run {
        println!("Not running cus told not to.");
        return;
    }

    let secret: u32 = rand::thread_rng().gen_range(1..101); // note: upper bound is exclusive

    println!("Here's the secret number, teehee: {}", secret);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Noice! You got it.");
                break;
            }
        }
    }
}

// note: constants require a type annotation
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn variables() {
    println!("\n~~VARIABLES~~\n");

    // constants
    println!("num of seconds in 3 hours: {}", THREE_HOURS_IN_SECONDS);

    // mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // chars
    let heart_cat = '😻';
    println!("Meow!: {}", heart_cat);

    // tuples
    let tup = (500, 'a', "meow");
    let (num, ch, meow) = tup;
    println!("num {}, char {}, meow {}", num, ch, meow);
    println!("whooo {}", tup.0);

    // arrays
    let something: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let all_three = [3; 5];
    println!("something[0] = {}", something[0]);
    println!("all_three[0] = {}", all_three[0]);
}

fn functions() {
    println!("\n~~FUNCTIONS~~\n");

    println!("only_five() = {}", only_five());
    println!("plus_one(4) = {}", plus_one(5));
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(5) = {}", is_even(5));
    println!("return_from_loop() = {}", return_from_loop());
    println!("for_loop():");
    for_loop()
}

fn only_five() -> i32 {
    5
}

fn plus_one(value: i32) -> i32 {
    value + 1
}

fn is_even(value: i32) -> bool {
    let even = if value % 2 == 0 { true } else { false };

    even
}

fn return_from_loop() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    result
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("element {}", element);
    }
}
