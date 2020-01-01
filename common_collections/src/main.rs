
// creating a new vector
let v: Vec<i32> = Vec::new();

let v = vec![1, 2, 4];


//how to update a vector
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(9);

// two ways to reference a vector

let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);

// looping over values in vectors

let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// iterate mutables to make changes to them

let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}


// Using Enums to Store Multiple Types
// Vectors can only store one Type of Data (think Arrays)
// You can use Enums, to store multiple types of data
// Then you can create a Vector to store the one type (enum), 
// Thus gaining access to multiple types

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec! [
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Text(String::from("blue")),
    SpreadSheetCell::Float(10.12),
]


fn main() {
    println!("Hello, world!");
}
