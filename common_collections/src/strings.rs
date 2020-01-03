// Learn Strings 


//this creates a string called s
let mut s = String::new();


//when we want to load a string with data
// we'll use the to_string() method

let data = "initial contents";

let s = data.to_string();

// you could also type
// let s = "initial contents".to_string()

// you could also type
// let s = String::from("initial contents");



fn main() {
    println!("Hello, world!");
}


// grow a string using the push method 

let mut foo = String::from("bar")
foo.push_str("stuff")

// this example works because push_str(); doesn't take control of the 
// object, it just uses the contents 
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);


// Another example
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
// Using the + operator to combine two String values into a new String value
// The string s3 will contain Hello, world! as a result of this code. The rea- son s1 is no longer valid after the addition and the reason we used a refer- ence to s2 has to do with the signature of the method that gets called when we use the + operator. The + operator uses the add method, whose signature looks something like this:
         fn add(self, s: &str) -> String {

//Covering format!();

// Say if you want to add a bunch of text together ex.

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s4 = s1 + "-" + &s2 + "-" + &s3

//OR 

let s4 = format!("{}-{}-{}", s1, s2, s3);



// indexing strings 
// this will not work

let s = String::from("hello");
let h = s[1];

// just don't even index strings like that, your head will hurt
// and you'll cry. INSTEAD Rust has a different way of doing it

for c in "hello".chars() {
    println!("{}", c);
}

// Just remember that RUST stores strings as UTF-8 instead of ASCII
// So UTF-8 is just going to give you the byte information whereas
// ASCII would've given you the key corresponding to that spot