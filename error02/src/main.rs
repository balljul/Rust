use std::net::IpAddr;

fn main() {

    let localhost: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded ip address should be valid");

    dbg!(&localhost);
    println!("Yay");
}
