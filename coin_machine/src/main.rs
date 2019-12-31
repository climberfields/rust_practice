#[#[derive(Debug)]]


// Make an enum of Coin because we will only have one of these 4 types
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// value_in_cents takes argument of coin from enum Coin, returns u32
fn value_in_cents(coin: Coin) -> u32 {
    //match works like if, however, doesn't return boolean
    match coin{
    // here are the match arms, match arms need pattern (Coin::) and code
    // => is the operator
    // the value of 1, 5, etc is the code
    // use curly brackets to run multiple lines of code
        Coin::Penny => {
            println! ("Lucky Penny!");1,
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
   
}

fn main() {
    println!("Hello, world!");
}

// This one will take into account States on Quarters

enum States {
    Alabama,
    Alaska,
    Arkansas,
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    // Quarter has a param of States because you can nest enums
    Quarter(States),
}

fn value_of_coin(coin: Coins) -> u32 {
    match coin {
        coin::Penny => 1,
        coin::Nickel => 5,
        coin::Dime => 10,
        // this calls quarter and when quarter is read the state will be read as well
        coin::Quarter(state) => {
            println!("This Quarter is from {:?}", state)
        } 25,
    }
}