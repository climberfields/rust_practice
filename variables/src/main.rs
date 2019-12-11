fn main() {
    /* let x = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    println!("The value of x is {}", x);

    let x = x * 2;
    println!("The value of x is {}", x);


    // constants should be written in caps

    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    let spaces = " ";

    let spaces = spaces.len();

    let f = 2.0;
    println!("{}", f);
    
    let g: f32 = 3.0;
    println!("{}", g);  
    */
    // setting up a tuple 
    let tup = (500, 6.4, 1);

    //destructuring a tuple
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);


    //accessing tuples using a . (period)
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_four = x.1;

    let one = x.2;

    println!("{}", one);

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    // this produces an error becuase the index has to be the same length
    // or greater than the list given
    println!("The value of element is: {}", element)

    
}

