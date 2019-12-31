enum Option<T> {
    Some(T),
    None,
}

fn main() {
    println!("Hello, world!");
}

// Function is supposed to return +1 onto a value if a value is present
// x is the argument, Option<T> is the enum type (is or isn't)
fn plus_one(x: Option<T>, ) -> u32 {
    // use match to match params
    match x {
        // None and some are the patterns
        // code states if None (value of X), return None
        // If Some(i) (integer), return Some(i + 1),
        None => None
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);



