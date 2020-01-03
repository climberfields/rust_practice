// Hashmaps

// hashmaps are constructed on the heap
// Hashmaps are also like vectors in that they have to be homogenous

use std::collections::HashMap;

let mut scores = HashMap::new();


scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

println!("the scores are {}-blue and {}-yellow", scores.Blue, scores.Yellow);



// OR you could write this as 

let teams = vec![String::from("blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];


// the type "HashMap" has to be used because RUST needs you to specify the type 
// for the parameters of Hashmap when we use underscors (_). Rust can infer the types
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

let field_name = String::from("favorite color")
let field_value = String::from("Blue")

let mut map = HashMap::new();
map.insert(field_name, field_value);

//We aren’t able to use the variables field_name and field_value 
//after they’ve been moved into the hash map with the call to insert.
//If we insert references to values into the hash map,
// the values won’t be moved into the hash map. The values that the references point to
// must be valid for at least as long as the hash map is valid.


// If we want to access the values in a HashMap we have to use the "get" method

use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = score.get(&team_name)

// the value returned will be Some(&10). 
// It's wrapped in Some because .get() returns Option(&V) 
// Option because get has to return something that is fitting a param(Option)
// Option(&V) is because it references the Value, but doesn't take ownership of it


// You can iterate over the key value pair just like a for loop

for (key, value) in &scores { //For each Key and Value in reference Scores (the keys would be Blue and Yellow)
    println!("{}: {}", key, value);
}

// This will print 
// Blue:10
// Yellow:50 


// You can updat HashMap values like such 

let mut s = HashMap::new();

scores.insert(String::from("blue"), 10)
scores.insert(String::from("blue"), 25)

println!("{:?}", scores);

//this will print 25 because 10 has been overwritten

//Inserting value where there is none 

let mut s = HashMap::new();

scores.insert(String::from("blue"), 10)

scores.entry(String::from("yellow")).or_insert(50);
scores.entry(String::from("blue")).or_insert(10);

println!("{:?}", scores);

//The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding 
//Entry key if that key exists, and if not,
// inserts the parameter as the new value for this key 
//and returns a mutable reference to the new value.

//Running the code in Listing 8-25 will print {"Yellow": 50, "Blue": 10}.
// The first call to entry will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a value
// The second call to entry will not change the hash map because the Blue team already has the value 10.

let text = "I am getting stronger in everything in life"

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);

// this will print {I: 1, am: 1, getting: 1, stronger: 1, in: 2, everything:1, life:1}
// breaking it down 

//for word in text.split_whitespace() 
//  LOOP THROUGH ALL VALUES IN VARIABLE "TEXT"
//let count = map.entry(word).or_insert(0);
//  WHEN DOING THIS ADD TO THE VARIABLE "COUNT" MAP.ENTRY(WORD) 
//  SO MAP IS A HASHMAP MEANING IT NEEDS TO BE NEEDS A KEY VALUE PAIR
//  SO COUNT WOULD THEN BECOME <_(WORD), _(VALUE OR 0 IN THIS CASE)
//*count += 1;
//  ADD 1 TO THE COUNT TO ALLOW IT TO UPDATE VALUES

// FROM THE BOOK
//The or_insert method actually returns a mutable reference (&mut V) to the value for this key.
// Here we store that mutable reference in the count variable, so in order to
//assign to that value, we must first dereference count using the asterisk (*).
// The mutable reference goes out of scope at the end of the for loop, 
//so all of these changes are safe and allowed by the borrowing rules.