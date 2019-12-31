/*
// module named network and in this network we have a function 
// function is called connect   
// to call this you would use network::connect
mod network {
    fn connect() {

    }
    mod server {
        fn connect() {

        }
    }
}

// module named client and function called connect
// to call this you could use client::connect
mod client {
    fn connect() {

    }
}

*/

// lets say you have mod network and decide to put client in there
// this would make sense because if the network had client functions it would be easier to read
// if you wanted to call this you could use network::client::connect
/*

mod network {
    fn connect() {

    }
    mod client{
        fn connect() {

        }
    }
}

*/ 
// mod client tells the file to look somewhere else
// mod client means (RUST search client.rs for contents and functions)
mod client; 

mod network ;









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
