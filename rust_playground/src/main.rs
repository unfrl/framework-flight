use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guessing_game(false);
    variables();
    functions();
    ownership();
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

fn ownership() {
    println!("\n~~OWNERSHIP~~\n");

    let mut s = String::from("hello");

    s.push_str(", world!!!!");

    println!("{}", s);

    let s1 = String::from("hello");
    // if .clone() had not been called, then s1 would have been invalidated,
    // resulting in a compiler error in the next println statement
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}!", s1, s2);

    // scalar types are stored on the stack -- do not need to call clone
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let some_string = String::from("hello"); // some_string comes into scope
    takes_ownership(some_string); // some_string's value moves into function & is no longer valid after this call

    let some_int = 5; // some_int comes into scope
    makes_copy(some_int); // some_int moves into the function, but since it's i32 it's a Copy and OK to use afterward

    let my_string = gives_ownership();
    let one_more = takes_and_gives_back(my_string);
    println!("{}", one_more);

    let (value, length) = calc_length(String::from("something"));
    println!("calc_length() = {} {}", value, length);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
} // some_string goes out of scope and drop is called -- backing memory is freed

fn makes_copy(some_int: i32) {
    println!("{}", some_int)
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(another_string: String) -> String {
    another_string
}

fn calc_length(some_string: String) -> (String, usize) {
    let length = some_string.len();

    (some_string, length)
}
