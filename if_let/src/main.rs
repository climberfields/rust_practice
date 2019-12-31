// _ () placeholder
// _ () allows for pattern and code cessation or recognition ex. below

let some_u8_value = Ou8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three" ),
    5 => println!("five" ),
    7 => println!("seven" ),

    // the _ () below will match any value. It will specify all cases not listed before it
    // the () is the unit value so it will return nothing as well
    _ => ()
}





fn main() {
    println!("Hello, world!");
}


//if let allows you to combine the IF and LET syntax
// this says if there is if the value of some_u8... is 3 print 3

let some_u8_value = Ou8;
match some_u8_value {
    Some(3) => println!("three" ),
    _ => (),
}

// or it could be written as 
if let Some(3) = some_u8_value{
println!("three" ).
}

