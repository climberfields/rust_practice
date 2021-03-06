fn main() {
    println!("Hello, world!");

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,

    
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,

        };
    user1.email = String::from("anotheremail@example.com")
    }
}



fn build_user(email::String, username::String) -> User {
    User{ 
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
   
let user2 = User {
    email: String::from("another#example.com")
    username: String::from("anotherusername")
    active: user1.active,
    sign_in_count: user1.sign_in_count,
    }
}



