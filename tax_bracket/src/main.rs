use std::io;

fn main() {
    println!("How much money do you make?");

    let mut income = String::new();

    let mut tax_bracket = [2, 3, 45];

    io::stdin().read_line(&mut income);

    println!("You make {} a year", income);
    println!("Your tax bracket is: {}", tax_bracket);
}
