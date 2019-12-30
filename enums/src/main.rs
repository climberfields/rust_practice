/* 

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;



fn route(ip_type: IpAddrKind) {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

} 
*/ 

// Or you can write

/*
enum IpAddr {
    V4(String),
    V6(String),
}


let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
// Enums can also be stored with different values for their datatype

enum IpAddr {
    V4(u8, u8, u8, u8)
    V6(String)
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"))

fn main() {
    println!("Hello, world!");
}
*/

enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}

let m = Message::Write(String::from("Hello"));
m.call();

enum Option<T> {
    Some(T),
    None,
}