extern crate utils;

fn main() {
    utils::client::connect();
    utils::network::connect();
    //utils::network::server::connect();
    let v = vec![3,5,1,6,7,7,3,1,1];
    println!("{}", utils::mean(&v));
    println!("{}", utils::mode(&v));
}
