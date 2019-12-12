fn main() {
    // a is the list that holds the values
    let a = [10, 20, 30, 40, 50];

    // making the index mutable so it can change values
    let mut index = 0;

    // when the loop executes it changes the value of index
    // the corresponding value change then loops through the variable "A"
    while index < 5 {
        println!("can't touch this {}!", a[index]);

        index = index + 1
    }
}